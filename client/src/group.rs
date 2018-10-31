use std::fmt;

use crate::error::Error;
use crate::jsonrpc::Client;
use serde_json::Value;

#[derive(Clone)]
pub struct Group {
    client: Client,
    name: String,
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
        let group_data: Option<GroupData> = self
            .client
            .request("getGroupData", Some(NameParams { name: &self.name }))?;

        Ok(group_data)
    }
}

pub struct GroupIterator {
    pub(crate) client: Client,
    pub(crate) group_names: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupData {
    communication: bool,
    frequency: u16,
    #[serde(rename = "groupId")]
    group_id: u64,
    hidden: bool,
    modulation: i64,
    name: String,
    #[serde(rename = "radioSet")]
    radio_set: bool,
    route: RouteData,
    start_time: u64,
    task: String, // TODO: enum
    tasks: Value, // TODO
    uncontrolled: bool,
    units: Vec<UnitData>, // TODO
    x: f64,
    y: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteData {
    points: Vec<PointData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointData {
    #[serde(rename = "ETA")]
    eta: i64,
    #[serde(rename = "ETA_locked")]
    eta_locked: bool,
    action: String,             // TODO: enum
    alt: i64,                   // f64?
    alt_type: String,           // TODO: enum
    formation_template: String, // TODO: enum?
    name: String,
    properties: Value,
    speed: f64,
    speed_locked: bool,
    task: Value, // TODO: enum
    #[serde(rename = "type")]
    kind: String, // TODO: enum
    x: f64,
    y: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitData {
    // AddPropAircraft
    // Radio
    alt: i64,         // f64?
    alt_type: String, // TODO: enum
    callsign: Value,  // TODO: propper struct
    // hardpoint_racks
    heading: f64,
    // livery_id
    name: String,
    // onboard_num
    payload: Value, // TODO
    // psi
    skill: String, // TODO: enum
    speed: f64,
    #[serde(rename = "type")]
    kind: String,
    #[serde(rename = "unitId")]
    unit_id: u64,
    x: f64,
    y: f64,
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
