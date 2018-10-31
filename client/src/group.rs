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
    #[serde(rename = "groupId")]
    pub id: u64,
    pub communication: bool,
    pub frequency: u16,
    pub hidden: bool,
    pub modulation: i64,
    pub name: String,
    #[serde(rename = "radioSet")]
    pub radio_set: bool,
    pub route: RouteData,
    pub start_time: u64,
    pub task: Task,
    pub tasks: Value, // TODO
    pub uncontrolled: bool,
    pub units: Vec<UnitData>,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteData {
    pub points: Vec<PointData>,
}

// known unimplemented properties: airdromeId, helipadId, formation_template
#[derive(Debug, Serialize, Deserialize)]
pub struct PointData {
    #[serde(rename = "ETA")]
    pub eta: f64,
    #[serde(rename = "ETA_locked")]
    pub eta_locked: bool,
    pub action: WaypointAction,
    pub alt: i64, // f64?
    pub alt_type: AltitudeType,
    pub name: String,
    pub properties: Value,
    pub speed: f64,
    pub speed_locked: bool,
    pub task: Value, // TODO: enum
    #[serde(rename = "type")]
    pub kind: WaypointType,
    pub x: f64,
    pub y: f64,
}

// known unimplemented properties: AddPropAircraft, Radio, hardpoint_racks, livery_id,
// onboard_num, psi
#[derive(Debug, Serialize, Deserialize)]
pub struct UnitData {
    #[serde(rename = "unitId")]
    pub id: u64,
    #[serde(rename = "type")]
    pub kind: String, // TODO: enum?
    pub name: String,
    pub alt: i64, // f64?
    pub alt_type: AltitudeType,
    pub callsign: Value, // TODO: propper struct
    pub heading: f64,
    pub payload: Value, // TODO
    pub skill: Skill,
    pub speed: f64,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Skill {
    Average,
    Client,
    Excellent,
    Good,
    High,
    Player,
}

impl Default for Skill {
    fn default() -> Self {
        Skill::Average
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AltitudeType {
    #[serde(rename = "BARO")]
    Baro,
    #[serde(rename = "RADIO")]
    Radio,
}

impl Default for AltitudeType {
    fn default() -> Self {
        AltitudeType::Baro
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WaypointType {
    Land,
    TakeOff,
    TakeOffParking,
    TakeOffParkingHot,
    #[serde(rename = "Turning Point")]
    TurningPoint,
}

impl Default for WaypointType {
    fn default() -> Self {
        WaypointType::TurningPoint
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WaypointAction {
    #[serde(rename = "Landing")]
    Land,
    #[serde(rename = "From Runway")]
    TakeOff,
    #[serde(rename = "From Parking Area")]
    TakeOffParking,
    #[serde(rename = "From Parking Area Hot")]
    TakeOffParkingHot,
    #[serde(rename = "Turning Point")]
    TurningPoint,
    #[serde(rename = "Fly Over Point")]
    FlyOverPoint,
}

impl Default for WaypointAction {
    fn default() -> Self {
        WaypointAction::TurningPoint
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Task {
    #[serde(rename = "Nothing")]
    Nothing,
    #[serde(rename = "AFAC")]
    AFAC,
    #[serde(rename = "Anti-ship Strike")]
    AntiShipStrike,
    #[serde(rename = "AWACS")]
    AWACS,
    #[serde(rename = "CAP")]
    CAP,
    #[serde(rename = "CAS")]
    CAS,
    #[serde(rename = "Escort")]
    Escort,
    #[serde(rename = "Fighter Sweep")]
    FighterSweep,
    #[serde(rename = "Ground Attack")]
    GroundAttack,
    #[serde(rename = "Intercept")]
    Intercept,
}

impl Default for Task {
    fn default() -> Self {
        Task::Nothing
    }
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
