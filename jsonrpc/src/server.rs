use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::thread;

use dcsjsonrpc_common::{Request, Response, RpcError};
use futures::sync::mpsc::{channel, Sender};
use serde_json::Value;
use tokio::codec::{Framed, LinesCodec};
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

type Queue = Arc<Mutex<VecDeque<PendingRequest>>>;
type Subscriptions = Arc<Mutex<HashMap<String, Vec<Sender<Outgoing>>>>>;

const JSONRPC_VERSION: &str = "2.0";

pub struct Server {
    queue: Queue,
    subscriptions: Subscriptions,
}

impl Server {
    pub fn new() -> Self {
        let server = Server {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
        };
        server.start();
        server
    }

    fn start(&self) {
        // Bind the server's socket.
        let addr = "127.0.0.1:7777".parse().unwrap();
        let listener = TcpListener::bind(&addr).expect("unable to bind TCP listener");

        let queue = self.queue.clone();
        let subs = self.subscriptions.clone();
        thread::spawn(move || {
            tokio::run_async(
                async move {
                    let mut incoming = listener.incoming();

                    while let Some(stream) = await!(incoming.next()) {
                        // TODO: unwrap
                        let stream = stream.unwrap();
                        handle(stream, queue.clone(), subs.clone());
                    }
                },
            );
        });
    }

    pub fn try_next(&self) -> Option<PendingRequest> {
        if let Ok(mut queue) = self.queue.try_lock() {
            queue.pop_front()
        } else {
            None
        }
    }

    pub fn broadcast(&self, channel: &str, params: Option<Value>) {
        if let Some(ref mut clients) = self.subscriptions.lock().unwrap().get_mut(channel) {
            for tx in clients.iter_mut() {
                // TODO: unwrap
                tx.try_send(Outgoing::Request(Request {
                    jsonrpc: JSONRPC_VERSION.to_string(),
                    method: channel.to_string(),
                    params: params.clone(),
                    id: None,
                }))
                .unwrap();
            }
        }
    }
}

fn handle(stream: TcpStream, queue: Queue, subs: Subscriptions) {
    tokio::spawn_async(
        async move {
            debug!("Client connected ...");

            let framed = Framed::new(stream, LinesCodec::new());
            let (mut sink, mut stream) = framed.split();

            let (tx, mut rx) = channel::<Outgoing>(128);
            tokio::spawn_async(
                async move {
                    // TODO: unwrap
                    while let Some(res) = await!(rx.next()) {
                        // TODO: unwrap?
                        let res = res.unwrap();
                        debug!("Responding with: {:?}", res);
                        let res = serde_json::to_string(&res).unwrap();
                        await!(sink.send_async(res));
                    }
                },
            );

            while let Some(line) = await!(stream.next()) {
                // TODO: unwrap
                let line = line.unwrap();
                let mut req = match serde_json::from_str::<Request>(&line) {
                    Ok(req) => {
                        if req.jsonrpc != JSONRPC_VERSION {
                            warn!("ignoring non JSON-RPC v2 request ...");
                            continue;
                        }
                        req
                    }
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

                match req.method.as_str() {
                    "subscribe" | "unsubscribe" => {
                        if let Some(params) = req.params.take() {
                            // TODO: unwrap
                            let params: SubParams = serde_json::from_value(params).unwrap();

                            let mut subs = subs.lock().unwrap();
                            match req.method.as_str() {
                                "subscribe" => {
                                    let subs = subs.entry(params.name).or_insert_with(Vec::new);
                                    subs.push(tx.clone());
                                }
                                "unsubscribe" => {
                                    subs.remove(&params.name);
                                }
                                _ => unreachable!(),
                            }

                            let mut req = PendingRequest {
                                req,
                                tx: tx.clone(),
                            };
                            req.success(json!("ok"));
                        } else {
                            // TODO: error
                        }
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
        },
    );
}

pub struct PendingRequest {
    pub req: Request,
    tx: Sender<Outgoing>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Outgoing {
    Request(Request),
    Response(Response),
}

impl PendingRequest {
    pub fn success(&mut self, result: Value) {
        if let Some(id) = self.req.id.take() {
            // TODO: unwrap
            self.tx
                .try_send(Outgoing::Response(Response::Success {
                    jsonrpc: JSONRPC_VERSION.to_string(),
                    result,
                    id,
                }))
                .unwrap();
        }
    }

    pub fn error(&mut self, error: String) {
        error!("Error Response: {} for {:?}", error, self.req);
        if let Some(id) = self.req.id.take() {
            // TODO: unwrap
            self.tx
                .try_send(Outgoing::Response(Response::Error {
                    jsonrpc: JSONRPC_VERSION.to_string(),
                    error: RpcError {
                        code: 1, // TODO: other codes?
                        message: error,
                        data: None, // TODO: provide data for some errors?
                    },
                    id,
                }))
                .unwrap();
        }
    }
}
