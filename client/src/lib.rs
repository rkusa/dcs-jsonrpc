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

    /// Displays the given `text` to all players for `display_time` seconds. `clear_view` defines
    /// whether existing messages will be overwritten (`true`) or whether the new message is
    /// stacked to existing ones (`false`).
    pub fn out_text(&self, text: &str, display_time: usize, clear_view: bool) -> Result<(), Error> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            text: &'a str,
            display_time: usize,
            clear_view: bool,
        }

        self.client.notification(
            "outText",
            Some(Params {
                text,
                display_time,
                clear_view,
            }),
        )
    }

    /// Removes the marker identified with `id` from the F10 map.
    pub fn remove_mark(&self, id: usize) -> Result<(), Error> {
        #[derive(Serialize)]
        struct Params {
            id: usize,
        }

        self.client.notification("removeMark", Some(Params { id }))
    }

    /// Instantiates the group that is identified with the given `name` (group names are unique).
    /// The group must exist, otherwise an [Error::GroupGone] error is returned.
    pub fn group(&self, name: &str) -> Result<Group, Error> {
        let group = Group::new(self.client.clone(), name);
        if group.exists()? {
            Ok(group)
        } else {
            Err(Error::GroupGone(Identifier::Name(name.to_string())))
        }
    }

    /// Returns an iterator that yields all groups for the given `coalition` and `category`.
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

    /// Adds a new group to the mission.
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

    /// Returns an endless iterator that will yield all future mission events.
    pub fn events(&self) -> Result<EventsIterator, Error> {
        let (tx, rx) = channel::<RawEvent>();
        self.client.add_subscription(tx);

        self.client.request::<(), String>("subscribe", None)?;

        Ok(EventsIterator {
            client: self.client.clone(),
            rx,
        })
    }

    /// Execute the given `lua` code within the mission environment.
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
