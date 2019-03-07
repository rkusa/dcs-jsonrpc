use std::fmt;
use std::hash::{Hash, Hasher};

use crate::group::GroupIterator;
use crate::jsonrpc::Client;
use crate::{Coalition, Country, Error, Group, Position};

#[derive(Clone, Serialize)]
pub struct Unit {
    #[serde(skip)]
    client: Client,
    name: String,
}

impl Unit {
    pub(crate) fn new<N: Into<String>>(client: Client, name: N) -> Self {
        Unit {
            client,
            name: name.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    fn request<R>(&self, method: &str) -> Result<R, Error>
    where
        for<'de> R: serde::Deserialize<'de>,
    {
        self.client
            .request::<_, Option<R>>(method, Some(&self))?
            .ok_or_else(|| Error::GroupGone(self.name.clone()))
    }

    pub fn exists(&self) -> Result<bool, Error> {
        self.client.request("unitExists", Some(&self))
    }

    pub fn position(&self) -> Result<Position, Error> {
        self.request("unitPosition")
    }

    pub fn group(&self) -> Result<Group, Error> {
        let name: String = self.request("unitGroup")?;
        Ok(Group::new(self.client.clone(), name))
    }

    pub fn infantry_load(&self, group: &Group) -> Result<(), Error> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            into: &'a str,
            load: &'a str,
        }

        self.client.notification(
            "unitInfantryLoad",
            Some(Params {
                into: &self.name,
                load: group.name(),
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
            unit: &'a str,
            unload: &'a str,
        }

        self.client.notification(
            "unitInfantryUnload",
            Some(Params {
                unit: &self.name,
                unload: group.name(),
            }),
        )
    }

    /// Requires a "Disembarking" task being setup for this unit and the provided `group` to work.
    pub fn infantry_smoke_unload_area(&self, group: &Group) -> Result<(), Error> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            unit: &'a str,
            smoke_for: &'a str,
        }

        self.client.notification(
            "unitInfantrySmokeUnloadArea",
            Some(Params {
                unit: &self.name,
                smoke_for: group.name(),
            }),
        )
    }

    pub fn loaded_groups(&self) -> Result<GroupIterator, Error> {
        let group_names: Vec<String> = self.request("unitLoadedGroups")?;

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

    pub fn life(&self) -> Result<f64, Error> {
        self.request("unitLife")
    }

    pub fn player_name(&self) -> Result<Option<String>, Error> {
        self.request("unitPlayerName")
    }

    pub fn is_player(&self) -> Result<bool, Error> {
        self.player_name().map(|r| r.is_some())
    }

    pub fn coalition(&self) -> Result<Coalition, Error> {
        self.request("unitCoalition")
    }

    pub fn country(&self) -> Result<Country, Error> {
        self.request("unitCountry")
    }

    pub fn destroy(self) -> Result<(), Error> {
        self.client.notification("unitDestory", Some(&self))
    }
}

impl PartialEq for Unit {
    fn eq(&self, other: &Unit) -> bool {
        self.name == other.name
    }
}

impl Eq for Unit {}

impl Hash for Unit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
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
        write!(f, "Unit {{ name: {} }}", self.name)
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unit {}", self.name)
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
