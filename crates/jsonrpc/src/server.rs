use std::collections::{HashMap, VecDeque};
use std::mem;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use dcsjsonrpc_common::{Notification, Request, Response, RpcError, Version};
use futures::channel::mpsc::{channel, Sender};
use futures::{FutureExt, SinkExt, StreamExt};
use serde_json::Value;
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Runtime;
use tokio_util::codec::{Framed, LinesCodec};

type Queue = Arc<Mutex<VecDeque<PendingRequest>>>;
type Subscriptions = Arc<Mutex<HashMap<String, Vec<Sender<Outgoing>>>>>;

pub struct Server {
    queue: Queue,
    subscriptions: Subscriptions,
    runtime: Runtime,
}

impl Server {
    pub fn start() -> Result<Self, anyhow::Error> {
        let server = Server {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
            runtime: Runtime::new()?,
        };

        let addr = "127.0.0.1:7777".parse().unwrap();

        let queue = server.queue.clone();
        let subs = server.subscriptions.clone();
        server.runtime.spawn(start(addr, queue, subs).map(|result| {
            if let Err(err) = result {
                error!("{}", err);
            }
            ()
        }));

        Ok(server)
    }

    pub fn stop(self) {
        mem::drop(self.runtime);
        info!("TCP server shut down");
    }

    pub fn try_next(&self) -> Option<PendingRequest> {
        if let Ok(mut queue) = self.queue.try_lock() {
            queue.pop_front()
        } else {
            None
        }
    }

    pub fn broadcast(&self, channel: &str, params: Option<Value>) {
        let forward = move |tx: &mut Sender<Outgoing>| {
            if let Err(err) = tx.try_send(Outgoing::Notification(Notification {
                jsonrpc: Version::V2,
                method: channel.to_string(),
                params: params.clone(),
            })) {
                error!("Error broadcasting message: {}", err);
            }
        };

        if let Some(ref mut clients) = self.subscriptions.lock().unwrap().get_mut(channel) {
            for tx in clients.iter_mut() {
                forward(tx);
            }
        }

        if let Some(ref mut clients) = self.subscriptions.lock().unwrap().get_mut("*") {
            for tx in clients.iter_mut() {
                forward(tx);
            }
        }
    }
}

async fn start(addr: SocketAddr, queue: Queue, subs: Subscriptions) -> Result<(), anyhow::Error> {
    let mut listener = TcpListener::bind(&addr).await?;

    loop {
        let stream = match listener.accept().await {
            Ok((stream, _)) => stream,
            Err(err) => {
                error!("Error establishing connection: {}", err);
                continue;
            }
        };

        tokio::spawn(handle_client(stream, queue.clone(), subs.clone()));
    }
}

async fn handle_client(stream: TcpStream, queue: Queue, subs: Subscriptions) {
    debug!("Client connected ...");

    let framed = Framed::new(stream, LinesCodec::new());
    let (mut sink, mut stream) = framed.split();

    let (mut tx, mut rx) = channel::<Outgoing>(128);
    tokio::spawn(async move {
        while let Some(res) = rx.next().await {
            debug!("Responding with: {:?}", res);
            match serde_json::to_string(&res) {
                Ok(res) => {
                    if let Err(err) = sink.send(res).await {
                        error!("Error sending message: {}", err);
                    }
                }
                Err(err) => {
                    error!("Error serializing outgoing message: {}", err);
                }
            }
        }

        debug!("Client sending loop closed");
    });

    while let Some(line) = stream.next().await {
        let line = match line {
            Ok(line) => line,
            Err(err) => {
                error!("Error reading next line from client: {}", err);
                break;
            }
        };

        let mut req = match serde_json::from_str::<Incoming>(&line) {
            Ok(req) => req,
            Err(err) => {
                warn!("Err: {}", err);
                warn!("ignoring invalid JSON-RPC v2 request ...");
                continue;
            }
        };

        debug!("Recv: {:?}", req);

        #[derive(Deserialize)]
        struct SubParams {
            name: String,
        }

        match req.method() {
            "subscribe" | "unsubscribe" => {
                let channel = if let Some(params) = req.take_params() {
                    let params: SubParams = match serde_json::from_value(params) {
                        Ok(params) => params,
                        Err(err) => {
                            if let Incoming::Request(mut req) = req {
                                error_response(
                                    &mut tx,
                                    &mut req,
                                    format!("Invalid subscribe/unsubscribe params: {}", err),
                                );
                            }
                            continue;
                        }
                    };
                    params.name
                } else {
                    "*".to_string()
                };

                let mut subs = subs.lock().unwrap();
                match req.method() {
                    "subscribe" => {
                        let subs = subs.entry(channel).or_insert_with(Vec::new);
                        subs.push(tx.clone());
                    }
                    "unsubscribe" => {
                        subs.remove(&channel);
                    }
                    _ => unreachable!(),
                }

                let mut req = PendingRequest {
                    req,
                    tx: tx.clone(),
                };
                req.success(json!("ok"));
            }
            _ => {
                let mut queue = queue.lock().unwrap();
                queue.push_back(PendingRequest {
                    req,
                    tx: tx.clone(),
                });
            }
        }
    }

    debug!("Client connection closed ...");
}

pub struct PendingRequest {
    pub req: Incoming,
    tx: Sender<Outgoing>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Outgoing {
    Notification(Notification),
    Response(Response),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Incoming {
    Request(Request),
    Notification(Notification),
}

impl Incoming {
    #[allow(unused)]
    pub fn jsonrpc(&self) -> Version {
        match *self {
            Incoming::Request(ref req) => req.jsonrpc,
            Incoming::Notification(ref req) => req.jsonrpc,
        }
    }

    pub fn method(&self) -> &str {
        match *self {
            Incoming::Request(ref req) => &req.method,
            Incoming::Notification(ref req) => &req.method,
        }
    }

    pub fn take_params(&mut self) -> Option<Value> {
        match *self {
            Incoming::Request(ref mut req) => req.params.take(),
            Incoming::Notification(ref mut req) => req.params.take(),
        }
    }
}

impl PendingRequest {
    pub fn success(&mut self, result: Value) {
        if let Incoming::Request(ref mut req) = self.req {
            if let Err(err) = self.tx.try_send(Outgoing::Response(Response::Success {
                jsonrpc: Version::V2,
                result,
                id: req.id.clone(),
            })) {
                error!("Error sending response: {}", err);
            }
        }
    }

    pub fn error(&mut self, error: String) {
        if let Incoming::Request(ref mut req) = self.req {
            error_response(&mut self.tx, req, error);
        }
    }
}

fn error_response(tx: &mut Sender<Outgoing>, req: &mut Request, error: String) {
    if let Err(err) = tx.try_send(Outgoing::Response(Response::Error {
        jsonrpc: Version::V2,
        error: RpcError {
            code: 1, // TODO: other codes?
            message: error,
            data: None, // TODO: provide data for some errors?
        },
        id: req.id.clone(),
    })) {
        error!("Error sending error response: {}", err);
    }
}
