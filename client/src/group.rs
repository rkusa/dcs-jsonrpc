use std::fmt;

use crate::coalition::Coalition;
use crate::country::Country;
use crate::error::Error;
use crate::jsonrpc::Client;
use serde_json::Value;

#[derive(Clone)]
pub struct Group {
    client: Client,
    name: String, // TODO: use group id instead?
}

enum_number!(GroupCategory {
    Airplane = 0,
    Helicopter = 1,
    Ground = 2,
    Ship = 3,
    Train = 4,
});

#[derive(Serialize)]
struct NameParams<'a> {
    name: &'a str,
}

impl Group {
    pub(crate) fn new<S: Into<String>>(client: Client, name: S) -> Self {
        Group {
            client,
            name: name.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn exists(&self) -> Result<bool, Error> {
        let exists: bool = self
            .client
            .request("groupExists", Some(NameParams { name: &self.name }))?;

        Ok(exists)
    }

    pub fn group_data(&self) -> Result<Option<GroupData>, Error> {
        self.client
            .request("groupData", Some(NameParams { name: &self.name }))
    }

    pub fn coalition(&self) -> Result<Option<Coalition>, Error> {
        self.client
            .request("groupCoalition", Some(NameParams { name: &self.name }))
    }

    pub fn country(&self) -> Result<Option<Country>, Error> {
        self.client
            .request("groupCountry", Some(NameParams { name: &self.name }))
    }

    pub fn category(&self) -> Result<Option<GroupCategory>, Error> {
        self.client
            .request("groupCategory", Some(NameParams { name: &self.name }))
    }

    pub fn activate(&self) -> Result<(), Error> {
        self.client
            .notification("groupActivate", Some(NameParams { name: &self.name }))
    }
}

pub struct GroupIterator {
    pub(crate) client: Client,
    pub(crate) group_names: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupData {
    pub communication: bool,
    pub frequency: u16,
    #[serde(rename = "groupId")]
    pub group_id: u64,
    pub hidden: bool,
    pub modulation: i64,
    pub name: String,
    #[serde(rename = "radioSet")]
    pub radio_set: bool,
    pub route: RouteData,
    pub start_time: u64,
    pub task: String, // TODO: enum
    pub tasks: Value, // TODO
    pub uncontrolled: bool,
    pub units: Vec<UnitData>, // TODO
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteData {
    pub points: Vec<PointData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointData {
    #[serde(rename = "ETA")]
    pub eta: f64,
    #[serde(rename = "ETA_locked")]
    pub eta_locked: bool,
    pub action: String,             // TODO: enum
    pub alt: i64,                   // f64?
    pub alt_type: String,           // TODO: enum
    pub formation_template: String, // TODO: enum?
    pub name: String,
    pub properties: Value,
    pub speed: f64,
    pub speed_locked: bool,
    pub task: Value, // TODO: enum
    #[serde(rename = "type")]
    pub kind: String, // TODO: enum
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitData {
    // AddPropAircraft
    // Radio
    pub alt: i64,         // f64?
    pub alt_type: String, // TODO: enum
    pub callsign: Value,  // TODO: propper struct
    // hardpoint_racks
    pub heading: f64,
    // livery_id
    pub name: String,
    // onboard_num
    pub payload: Value, // TODO
    // psi
    pub skill: String, // TODO: enum
    pub speed: f64,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(rename = "unitId")]
    pub unit_id: u64,
    pub x: f64,
    pub y: f64,
}

impl Iterator for GroupIterator {
    type Item = Group;

    fn next(&mut self) -> Option<Self::Item> {
        self.group_names
            .pop()
            .map(|name| Group::new(self.client.clone(), name))
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
