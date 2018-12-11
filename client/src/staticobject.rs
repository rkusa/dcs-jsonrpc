use std::borrow::Cow;
use std::fmt;

use crate::jsonrpc::Client;
use crate::{Identifier, Error};

#[derive(Clone)]
pub struct Static {
    client: Client,
    id: Identifier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StaticCategory {
    Cargos,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StaticData {
    #[serde(rename = "unitId")]
    pub id: u64,
    #[serde(rename = "type")]
    pub category: StaticCategory,
    pub name: String,
    pub shape_name: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub heading: f64,
    pub x: f64,
    pub y: f64,
    #[serde(default, rename = "canCargo")]
    pub can_cargo: bool,
    #[serde(default)]
    pub mass: u32, // in kg
}

//enum_number!(StaticCategory {
//    Void = 0,
//    Unit = 1,
//    Weapon = 2,
//    Static = 3,
//    Base = 4,
//    Scenery = 5,
//    Cargo = 6,
//});

impl Static {
    pub(crate) fn new<I: Into<Identifier>>(client: Client, id: I) -> Self {
        Static {
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
            .ok_or_else(|| Error::StaticGone(self.id.clone()))
    }

    pub fn id(&self) -> Result<usize, Error> {
        match self.id {
            Identifier::ID(id) => Ok(id),
            Identifier::Name(_) => self.request("staticID"),
        }
    }

    pub fn name(&self) -> Result<Cow<'_, str>, Error> {
        match self.id {
            Identifier::ID(_) => self.request("staticName").map(Cow::Owned),
            Identifier::Name(ref name) => Ok(Cow::Borrowed(name)),
        }
    }

    pub fn exists(&self) -> Result<bool, Error> {
        self.client.request("staticExists", Some(&self.id))
    }
}

impl fmt::Debug for Static {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Static {{ id: {} }}", self.id)
    }
}

impl fmt::Display for Static {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Static {}", self.id)
    }
}

impl Default for StaticCategory {
    fn default() -> Self {
        StaticCategory::Cargos
    }
}
