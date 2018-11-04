#![warn(rust_2018_idioms)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
mod macros;
mod airbase;
mod coalition;
mod country;
mod error;
mod event;
mod group;
mod identifier;
mod jsonrpc;
mod position;
mod scenery;
mod staticobject;
mod unit;
mod weapon;

use std::net::ToSocketAddrs;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::{Duration, Instant};

pub use self::airbase::Airbase;
pub use self::coalition::Coalition;
pub use self::country::Country;
pub use self::error::Error;
pub use self::event::Event;
use self::event::RawEvent;
pub use self::group::*;
pub use self::identifier::Identifier;
pub use self::position::Position;
pub use self::scenery::Scenery;
pub use self::staticobject::Static;
pub use self::unit::Unit;
pub use self::weapon::Weapon;
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

    pub fn group(&self, name: &str) -> Result<Group, Error> {
        let group = Group::new(self.client.clone(), name);
        if group.exists()? {
            Ok(group)
        } else {
            Err(Error::GroupGone(Identifier::Name(name.to_string())))
        }
    }

    pub fn groups(
        &self,
        coalition: Coalition,
        category: Option<GroupCategory>,
    ) -> Result<GroupIterator, Error> {
        #[derive(Serialize)]
        struct Params {
            coalition: Coalition,
            #[serde(skip_serializing_if = "Option::is_none")]
            category: Option<GroupCategory>,
        }

        let group_names: Vec<String> = self.client.request(
            "getGroups",
            Some(Params {
                coalition,
                category,
            }),
        )?;

        Ok(GroupIterator {
            client: self.client.clone(),
            group_names,
        })
    }

    pub fn add_group(
        &self,
        country: Country,
        category: GroupCategory,
        data: GroupData,
    ) -> Result<Group, Error> {
        #[derive(Serialize)]
        struct Params {
            country: Country,
            category: GroupCategory,
            data: GroupData,
        }

        let name = data.name.clone();
        self.client.notification(
            "addGroup",
            Some(Params {
                country,
                category,
                data,
            }),
        )?;

        let group = Group::new(self.client.clone(), name);
        let started = Instant::now();
        while !group.exists()? {
            if started.elapsed() > Duration::from_secs(1) {
                return Err(Error::AddGroupTimeout);
            }
            thread::sleep(Duration::from_millis(100));
        }

        Ok(group)
    }

    pub fn events(&self) -> Result<EventsIterator, Error> {
        let (tx, rx) = channel::<RawEvent>();
        self.client.add_subscription(tx);

        self.client.request::<(), String>("subscribe", None)?;

        Ok(EventsIterator {
            client: self.client.clone(),
            rx,
        })
    }

    pub fn execute<R>(&self, lua: &str) -> Result<R, Error>
    where
        for<'de> R: serde::Deserialize<'de>,
    {
        #[derive(Serialize)]
        struct Params<'a> {
            lua: &'a str,
        }

        self.client.request("execute", Some(Params { lua }))
    }
}

pub struct EventsIterator {
    client: jsonrpc::Client,
    rx: Receiver<RawEvent>,
}

impl Iterator for EventsIterator {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.rx
            .recv()
            .ok()
            .map(|ev| ev.into_event(self.client.clone()))
    }
}
