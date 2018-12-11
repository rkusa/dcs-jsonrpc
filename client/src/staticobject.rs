use std::fmt;

use crate::jsonrpc::Client;
use crate::{Error};

#[derive(Clone, Serialize)]
pub struct Static {
    #[serde(skip)]
    client: Client,
    name: String,
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
    pub(crate) fn new<N: Into<String>>(client: Client, name: N) -> Self {
        Static {
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
            .ok_or_else(|| Error::StaticGone(self.name.clone()))
    }

    pub fn id(&self) -> Result<usize, Error> {
        self.request("staticID")
    }

    pub fn exists(&self) -> Result<bool, Error> {
        self.client.request("staticExists", Some(&self))
    }
}

impl fmt::Debug for Static {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Static {{ name: {} }}", self.name)
    }
}

impl fmt::Display for Static {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Static {}", self.name)
    }
}

impl Default for StaticCategory {
    fn default() -> Self {
        StaticCategory::Cargos
    }
}
