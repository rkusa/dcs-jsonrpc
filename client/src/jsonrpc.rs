use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use crate::error::Error;
use crate::event::RawEvent;
use dcsjsonrpc_common::{Notification, Request, Response, Version, ID};
use heck::CamelCase;
use serde_json::Value;

#[derive(Clone)]
pub struct Client {
    tx: mpsc::Sender<Vec<u8>>,
    // TODO: remove pending responses after a certain amount of time
    pending: Arc<Mutex<HashMap<ID, mpsc::Sender<Response>>>>,
    next_id: Arc<Mutex<i64>>,
    subscriptions: Arc<Mutex<Vec<mpsc::Sender<RawEvent>>>>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Incoming {
    Notification(Notification),
    Response(Response),
}

impl Client {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self, Error> {
        let mut stream = TcpStream::connect(addr)?;
        stream.set_nodelay(true)?;
        let rd = stream.try_clone()?;
        let mut rd = BufReader::new(rd);

        let pending = Arc::new(Mutex::new(HashMap::new()));
        let subs = Arc::new(Mutex::new(Vec::new()));
        let (tx, rx) = mpsc::channel::<Vec<u8>>();
        let client = Client {
            tx: tx,
            pending: pending.clone(),
            next_id: Arc::new(Mutex::new(0)),
            subscriptions: subs.clone(),
        };

        thread::spawn(move || loop {
            let mut line = String::new();
            if let Err(err) = rd.read_line(&mut line) {
                eprintln!("Error reading from TCP stream: {}", err);
                break;
            }

            let res: Incoming = match serde_json::from_str(&line) {
                Ok(res) => res,
                Err(err) => {
                    eprintln!(
                        "Error deserializing response: {}\nRaw response: {}",
                        err, line
                    );
                    continue;
                }
            };

            match res {
                Incoming::Response(res) => {
                    let id = match res {
                        Response::Success { ref id, .. } => id,
                        Response::Error { ref id, .. } => id,
                    };
                    let mut pending = pending.lock().unwrap();
                    if let Some(tx) = pending.remove(&id) {
                        if let Err(err) = tx.send(res) {
                            eprintln!("Error routing response: {}", err);
                        }
                    } else {
                        eprintln!("No pending response for id {} found", id);
                    }
                }
                Incoming::Notification(Notification { method, params, .. }) => {
                    if let Some(params) = params {
                        let variant = method.to_camel_case();
                        let mut map = serde_json::Map::new();
                        map.insert(variant, params);
                        let params = Value::Object(map);

                        let event: RawEvent = match serde_json::from_value(params) {
                            Ok(ev) => ev,
                            Err(err) => {
                                eprintln!("Error deserializing event: {}", err);
                                continue;
                            }
                        };

                        let mut subs = subs.lock().unwrap();
                        subs.retain(|tx| tx.send(event.clone()).is_ok());
                    }
                }
            }
        });

        thread::spawn(move || {
            let mut forwad = || -> Result<(), Error> {
                let mut data = rx.recv()?;
                data.push('\n' as u8);
                stream.write(&data)?;
                Ok(())
            };

            loop {
                if let Err(err) = forwad() {
                    eprintln!("Error sending request: {}", err);
                    break;
                }
            }
        });

        Ok(client)
    }

    #[allow(unused)]
    pub fn request<P, R>(&self, method: &str, params: Option<P>) -> Result<R, Error>
    where
        P: serde::Serialize,
        for<'de> R: serde::Deserialize<'de>,
    {
        let req = Request {
            jsonrpc: Version::V2,
            method: method.to_string(),
            params: params
                .map(|p| serde_json::to_value(p))
                .map_or(Ok(None), |r| r.map(Some))?,
            id: self.get_next_id(),
        };

        let (tx, rx) = mpsc::channel();
        {
            let mut pending = self.pending.lock().unwrap();
            pending.insert(req.id.clone(), tx);
        }

        let data = serde_json::to_vec(&req)?;
        self.tx.send(data)?;

        let res = rx.recv()?;
        match res {
            Response::Success { result, .. } => {
                let res: R = serde_json::from_value(result)?;
                Ok(res)
            }
            Response::Error { error, .. } => Err(error.into()),
        }
    }

    #[allow(unused)]
    pub fn notification<P>(&self, method: &str, params: Option<P>) -> Result<(), Error>
    where
        P: serde::Serialize,
    {
        let notification = Notification {
            jsonrpc: Version::V2,
            method: method.to_string(),
            params: params
                .map(|p| serde_json::to_value(p))
                .map_or(Ok(None), |r| r.map(Some))?,
        };

        let data = serde_json::to_vec(&notification)?;
        self.tx.send(data)?;

        Ok(())
    }

    fn get_next_id(&self) -> ID {
        let mut next_id = self.next_id.lock().unwrap();
        let id = if *next_id == i64::max_value() {
            0
        } else {
            *next_id + 1
        };
        *next_id = id;
        ID::Number(id)
    }

    pub(crate) fn add_subscription(&self, tx: mpsc::Sender<RawEvent>) {
        let mut subs = self.subscriptions.lock().unwrap();
        subs.push(tx);
    }
}
