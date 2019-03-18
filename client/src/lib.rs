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
mod jsonrpc;
mod menu;
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
pub use self::menu::*;
pub use self::position::Position;
pub use self::scenery::Scenery;
pub use self::staticobject::*;
pub use self::unit::*;
pub use self::weapon::Weapon;
pub use dcsjsonrpc_common::*;
use std::cell::RefCell;

pub struct Client<C = usize>
where
    for<'de> C: serde::Serialize + serde::Deserialize<'de>,
{
    client: jsonrpc::Client,
    mark: std::marker::PhantomData<C>,
}

impl<C> Client<C>
where
    for<'de> C: serde::Serialize + serde::Deserialize<'de>,
{
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self, Error> {
        Ok(Client {
            client: jsonrpc::Client::connect(addr)?,
            mark: std::marker::PhantomData,
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
            Err(Error::GroupGone(name.to_string()))
        }
    }

    pub fn group_unchecked(&self, name: &str) -> Group {
        Group::new(self.client.clone(), name)
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

    pub fn airbase(&self, name: &str) -> Result<Airbase, Error> {
        let airbase = Airbase::new(self.client.clone(), name);
        if airbase.exists()? {
            Ok(airbase)
        } else {
            Err(Error::NonExistent)
        }
    }

    pub fn static_object(&self, name: &str) -> Result<Static, Error> {
        let staticobj = Static::new(self.client.clone(), name);
        if staticobj.exists()? {
            Ok(staticobj)
        } else {
            Err(Error::StaticGone(name.to_string()))
        }
    }

    /// Adds a new group to the mission.
    pub fn add_group(
        &self,
        country: Country,
        category: GroupCategory,
        data: &GroupData,
    ) -> Result<Group, Error> {
        #[derive(Serialize)]
        struct Params<'a> {
            country: Country,
            category: GroupCategory,
            data: &'a GroupData,
        }

        let name = match data {
            GroupData::Aircraft(AircraftGroupData { ref name, .. })
            | GroupData::Ground(GroundGroupData { ref name, .. }) => name.clone(),
        };
        self.client.notification(
            "addGroup",
            Some(Params {
                country,
                category,
                data,
            }),
        )?;

        let mut group = Group::new(self.client.clone(), name);
        group.country = RefCell::new(Some(country));
        group.category = RefCell::new(Some(category));
        group.data = RefCell::new(Some(data.clone()));
        let started = Instant::now();
        while !group.exists()? {
            if started.elapsed() > Duration::from_secs(1) {
                return Err(Error::AddGroupTimeout);
            }
            thread::sleep(Duration::from_millis(100));
        }

        Ok(group)
    }

    /// Adds a new static object to the mission.
    pub fn add_static(&self, country: Country, data: &StaticData) -> Result<Static, Error> {
        #[derive(Serialize)]
        struct Params<'a> {
            country: Country,
            data: &'a StaticData,
        }

        let name = data.name.clone();
        self.client
            .notification("addStatic", Some(Params { country, data }))?;

        let staticobj = Static::new(self.client.clone(), name);
        let started = Instant::now();
        while !staticobj.exists()? {
            if started.elapsed() > Duration::from_secs(1) {
                return Err(Error::AddStaticTimeout);
            }
            thread::sleep(Duration::from_millis(100));
        }

        Ok(staticobj)
    }

    /// Returns an endless iterator that will yield all future mission events.
    pub fn events(&self) -> Result<EventsIterator<C>, Error> {
        let (tx, rx) = channel::<RawEvent>();
        self.client.add_subscription(tx);

        self.client.request::<(), String>("subscribe", None)?;

        Ok(EventsIterator {
            client: self.client.clone(),
            rx,
            mark: self.mark,
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

    pub fn add_submenu(&self, name: &str) -> Result<SubMenu<C>, Error> {
        crate::menu::add_submenu(&self.client, name, None)
    }

    pub fn add_group_submenu(&self, group: &Group, name: &str) -> Result<GroupSubMenu<C>, Error> {
        let id = group.id()?;
        crate::menu::add_group_submenu(&self.client, id, name, None)
    }

    pub fn add_coalition_submenu(
        &self,
        coalition: Coalition,
        name: &str,
    ) -> Result<CoalitionSubMenu<C>, Error> {
        crate::menu::add_coalition_submenu(&self.client, coalition, name, None)
    }

    pub fn add_command(&self, name: &str, command: C) -> Result<MenuEntry, Error> {
        crate::menu::add_command(&self.client, name, None, command)
    }

    pub fn add_group_command(
        &self,
        group: &Group,
        name: &str,
        command: C,
    ) -> Result<GroupMenuEntry, Error> {
        let id = group.id()?;
        crate::menu::add_group_command(&self.client, id, name, None, command)
    }

    pub fn add_coalition_command(
        &self,
        coalition: Coalition,
        name: &str,
        command: C,
    ) -> Result<CoalitionMenuEntry, Error> {
        crate::menu::add_coalition_command(&self.client, coalition, name, None, command)
    }

    pub fn zone(&self, name: &str) -> Result<Zone, Error> {
        #[derive(Serialize)]
        struct Params<'a> {
            name: &'a str,
        }

        let mut zone: Option<Zone> = self.client.request("getZone", Some(Params { name }))?;
        match zone.take() {
            Some(zone) => Ok(zone),
            None => Err(Error::ZoneGone(name.to_string())),
        }
    }

    pub fn zones(&self) -> Result<Vec<String>, Error> {
        self.client.request::<(), Vec<String>>("getZones", None)
    }

    pub fn get_user_flag(&self, flag: &str) -> Result<u16, Error> {
        #[derive(Serialize)]
        struct Params<'a> {
            flag: &'a str,
        }

        self.client.request("getUserFlag", Some(Params { flag }))
    }

    pub fn set_user_flag(&self, flag: &str, value: u16) -> Result<(), Error> {
        #[derive(Serialize)]
        struct Params<'a> {
            flag: &'a str,
            value: u16,
        }

        self.client
            .notification("setUserFlag", Some(Params { flag, value }))
    }
}

#[derive(Debug, Deserialize)]
pub struct Zone {
    #[serde(rename = "point")]
    pub position: Position,
    pub radius: f64, // in m
}

pub struct EventsIterator<C> {
    client: jsonrpc::Client,
    rx: Receiver<RawEvent>,
    mark: std::marker::PhantomData<C>,
}

impl<C> Clone for Client<C>
where
    for<'de> C: serde::Serialize + serde::Deserialize<'de>,
{
    fn clone(&self) -> Self {
        Client {
            client: self.client.clone(),
            mark: self.mark.clone(),
        }
    }
}

impl<C> Iterator for EventsIterator<C>
where
    for<'de> C: serde::Serialize + serde::Deserialize<'de>,
{
    type Item = Event<C>;

    fn next(&mut self) -> Option<Self::Item> {
        self.rx
            .recv()
            .ok()
            .and_then(|ev| match ev.into_event(self.client.clone()) {
                Ok(ev) => Some(ev),
                Err(err) => {
                    // TODO: remove eprintln ?
                    eprintln!("Error deserializing command: {}", err);
                    None
                }
            })
    }
}
