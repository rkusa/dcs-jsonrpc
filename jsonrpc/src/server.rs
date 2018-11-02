use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::{mem, thread};

use crate::error::Error;
use dcsjsonrpc_common::{Notification, Request, Response, RpcError, Version};
use futures::sync::mpsc::{channel, Sender};
use futures::sync::oneshot;
use serde_json::Value;
use tokio::codec::{Framed, LinesCodec};
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

type Queue = Arc<Mutex<VecDeque<PendingRequest>>>;
type Subscriptions = Arc<Mutex<HashMap<String, Vec<Sender<Outgoing>>>>>;

pub struct Server {
    queue: Queue,
    subscriptions: Subscriptions,
    shutdown: oneshot::Sender<()>,
}

impl Server {
    pub fn start() -> Result<Self, Error> {
        let (tx, rx) = oneshot::channel::<()>();
        let server = Server {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
            shutdown: tx,
        };

        let addr = "127.0.0.1:7777".parse().unwrap();
        let listener = TcpListener::bind(&addr)?;

        let queue = server.queue.clone();
        let subs = server.subscriptions.clone();
        thread::spawn(move || {
            tokio::run_async(
                async move {
                    let mut incoming = Shutdownable::new(listener.incoming(), rx);
                    let clients: Arc<
                        Mutex<HashMap<usize, oneshot::Sender<()>>>,
                    > = Arc::new(Mutex::new(HashMap::new()));
                    let mut next_ix = 1;

                    while let Some(stream) = await!(incoming.next()) {
                        let stream = match stream {
                            Ok(stream) => stream,
                            Err(err) => {
                                error!("Error establishing connection: {}", err);
                                continue;
                            }
                        };

                        // client shutdown channel
                        let (tx, rx) = oneshot::channel();
                        let ix = next_ix;
                        next_ix += 1;
                        clients.lock().unwrap().insert(ix, tx);

                        let clients = clients.clone();
                        let queue = queue.clone();
                        let subs = subs.clone();
                        tokio::spawn_async(
                            async move {
                                await!(handle_client(stream, queue, subs, rx));

                                clients.lock().unwrap().remove(&ix);
                            },
                        );
                    }

                    let mut clients = clients.lock().unwrap();
                    let clients = mem::replace(&mut *clients, HashMap::new());
                    for (_, client) in clients {
                        let _ = client.send(()).unwrap();
                    }

                    info!("TCP server shut down");
                },
            );
        });

        Ok(server)
    }

    pub fn stop(self) {
        self.shutdown.send(()).unwrap();
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

async fn handle_client(
    stream: TcpStream,
    queue: Queue,
    subs: Subscriptions,
    shutdown: oneshot::Receiver<()>,
) {
    debug!("Client connected ...");

    let framed = Framed::new(stream, LinesCodec::new());
    let (mut sink, stream) = framed.split();

    let (mut tx, mut rx) = channel::<Outgoing>(128);
    tokio::spawn_async(
        async move {
            while let Some(res) = await!(rx.next()) {
                // receive stream has error type (), ie, it will not throw an error ever,
                // thus unwrap is fine
                let res = res.unwrap();

                debug!("Responding with: {:?}", res);
                match serde_json::to_string(&res) {
                    Ok(res) => {
                        await!(sink.send_async(res));
                    }
                    Err(err) => {
                        error!("Error serializing outgoing message: {}", err);
                    }
                }
            }

            debug!("Client sending loop closed");
        },
    );

    let mut stream = Shutdownable::new(stream, shutdown);
    while let Some(line) = await!(stream.next()) {
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

#[must_use = "streams do nothing unless polled"]
#[derive(Debug)]
pub struct Shutdownable<S>
where
    S: Stream,
{
    inner: Box<S>,
    shutdown: oneshot::Receiver<()>,
}

impl<S> Shutdownable<S>
where
    S: Stream,
{
    pub(crate) fn new(listener: S, shutdown: oneshot::Receiver<()>) -> Self {
        Shutdownable {
            inner: Box::new(listener),
            shutdown,
        }
    }
}

impl<S> Stream for Shutdownable<S>
where
    S: Stream,
{
    type Item = S::Item;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use futures::Async;

        if self.shutdown.poll() != Ok(Async::NotReady) {
            return Ok(Async::Ready(None));
        }

        self.inner.poll()
    }
}
