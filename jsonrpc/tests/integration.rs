extern crate dcsjsonrpc_common;
#[macro_use]
extern crate serde_json;

use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use dcsjsonrpc_common::{Request, ID};
use hlua51::Lua;
use serde_json::Value;

#[test]
fn test_integration() {
    thread::spawn(|| {
        let mut lua = Lua::new();
        lua.openlibs();
        lua.execute::<()>(include_str!("./integration.lua"))
            .unwrap();
    });

    thread::sleep(Duration::from_millis(100));

    let mut stream = TcpStream::connect("127.0.0.1:7777").unwrap();
    stream.set_nodelay(true).unwrap();
    let mut rd = BufReader::new(stream.try_clone().unwrap());

    let req = Request {
        jsonrpc: "2.0".to_string(),
        method: "health".to_string(),
        params: None,
        id: Some(ID::Number(1)),
    };
    serde_json::to_writer(&stream, &req).unwrap();
    write!(stream, "\n");

    let mut line = String::new();
    rd.read_line(&mut line).unwrap();
    let json: Value = serde_json::from_str(&line).unwrap();

    assert_eq!(json, json!({"jsonrpc":"2.0","result":"ok","id":1}),);
}
