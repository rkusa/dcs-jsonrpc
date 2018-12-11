use std::borrow::Cow;
use std::fmt;

use crate::group::GroupIterator;
use crate::jsonrpc::Client;
use crate::{Error, Group, Identifier, Position};

#[derive(Clone)]
pub struct Unit {
    client: Client,
    pub(crate) id: Identifier,
}

impl Unit {
    pub(crate) fn new<I: Into<Identifier>>(client: Client, id: I) -> Self {
        Unit {
            client,
            id: id.into(),
        }
    }

    fn request<R>(&self, method: &str) -> Result<R, Error>
    where
        for<'de> R: serde::Deserialize<'de>,
    {
        self.client
            .request::<_, Option<R>>(method, Some(&self.id))?
            .ok_or_else(|| Error::GroupGone(self.id.clone()))
    }

    pub fn name(&self) -> Result<Cow<'_, str>, Error> {
        match self.id {
            Identifier::ID(_) => self.request("unitName").map(Cow::Owned),
            Identifier::Name(ref name) => Ok(Cow::Borrowed(name)),
        }
    }

    pub fn position(&self) -> Result<Position, Error> {
        self.request("unitPosition")
    }

    pub fn infantry_load(&self, group: &Group) -> Result<(), Error> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            into: &'a Identifier,
            load: &'a Identifier,
        }

        self.client.notification(
            "unitInfantryLoad",
            Some(Params {
                into: &self.id,
                load: &group.id,
            }),
        )
    }

    pub fn infantry_capacity(&self) -> Result<u32, Error> {
        self.request("unitInfantryCapacity")
    }

    pub fn infantry_loaded(&self) -> Result<u32, Error> {
        self.request("unitInfantryLoaded")
    }

    pub fn infantry_unload(&self, group: &Group) -> Result<(), Error> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            unit: &'a Identifier,
            unload: &'a Identifier,
        }

        self.client.notification(
            "unitInfantryUnload",
            Some(Params {
                unit: &self.id,
                unload: &group.id,
            }),
        )
    }

    /// Requires a "Disembarking" task being setup for this unit and the provided `group` to work.
    pub fn infantry_smoke_unload_area(&self, group: &Group) -> Result<(), Error> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            unit: &'a Identifier,
            smoke_for: &'a Identifier,
        }

        self.client.notification(
            "unitInfantrySmokeUnloadArea",
            Some(Params {
                unit: &self.id,
                smoke_for: &group.id,
            }),
        )
    }

    pub fn loaded_groups(&self) -> Result<GroupIterator, Error> {
        let group_names: Vec<String> = self.client.request("unitLoadedGroups", Some(&self.id))?;

        Ok(GroupIterator {
            client: self.client.clone(),
            group_names,
        })
    }

    pub fn is_airborne(&self) -> Result<bool, Error> {
        self.request("unitIsAirborne")
    }

    pub fn orientation(&self) -> Result<Orientation, Error> {
        self.request("unitOrientation")
    }
}

#[derive(Debug, Deserialize)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Deserialize)]
pub struct Orientation {
    pub p: Vector,
    pub x: Vector,
    pub y: Vector,
    pub z: Vector,
}

pub struct UnitIterator {
    pub(crate) client: Client,
    pub(crate) unit_names: Vec<String>,
}

impl fmt::Debug for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unit {{ id: {} }}", self.id)
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unit {}", self.id)
    }
}

impl Iterator for UnitIterator {
    type Item = Unit;

    fn next(&mut self) -> Option<Self::Item> {
        self.unit_names
            .pop()
            .map(|name| Unit::new(self.client.clone(), name))
    }
}
