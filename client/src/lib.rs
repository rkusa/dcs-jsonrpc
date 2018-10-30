#![warn(rust_2018_idioms)]

#[macro_use]
extern crate serde_derive;

mod error;
mod group;
mod jsonrpc;

use std::net::ToSocketAddrs;

pub use self::error::Error;
pub use self::group::Group;
pub use dcsjsonrpc_common::*;

pub struct Client {
    client: jsonrpc::Client,
}

impl Client {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self, Error> {
        Ok(Client {
            client: jsonrpc::Client::connect(addr)?,
        })
    }

    pub fn group(&self, name: &str) -> Group {
        Group::new(self.client.clone(), name)
    }
}
