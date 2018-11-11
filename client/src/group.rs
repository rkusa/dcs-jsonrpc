use std::borrow::Cow;
use std::fmt;

use crate::jsonrpc::Client;
use crate::unit::UnitIterator;
use crate::{Coalition, Country, Error, Identifier};
use serde_json::Value;

#[derive(Clone)]
pub struct Group {
    client: Client,
    id: Identifier,
}

enum_number!(GroupCategory {
    Airplane = 0,
    Helicopter = 1,
    Ground = 2,
    Ship = 3,
    Train = 4,
});

impl Group {
    pub(crate) fn new<I: Into<Identifier>>(client: Client, id: I) -> Self {
        Group {
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

    pub fn id(&self) -> Result<usize, Error> {
        match self.id {
            Identifier::ID(id) => Ok(id),
            Identifier::Name(_) => self.request("groupID"),
        }
    }

    pub fn name(&self) -> Result<Cow<'_, str>, Error> {
        match self.id {
            Identifier::ID(_) => self.request("groupName").map(Cow::Owned),
            Identifier::Name(ref name) => Ok(Cow::Borrowed(name)),
        }
    }

    pub fn exists(&self) -> Result<bool, Error> {
        self.client.request("groupExists", Some(&self.id))
    }

    pub fn data(&self) -> Result<Option<GroupData>, Error> {
        self.client.request("groupData", Some(&self.id))
    }

    pub fn coalition(&self) -> Result<Coalition, Error> {
        self.request("groupCoalition")
    }

    pub fn country(&self) -> Result<Country, Error> {
        self.request("groupCountry")
    }

    pub fn category(&self) -> Result<GroupCategory, Error> {
        self.request("groupCategory")
    }

    pub fn activate(&self) -> Result<(), Error> {
        self.client.notification("groupActivate", Some(&self.id))
    }

    pub fn units(&self) -> Result<UnitIterator, Error> {
        let unit_names: Vec<String> = self.client.request("groupUnits", Some(&self.id))?;

        Ok(UnitIterator {
            client: self.client.clone(),
            unit_names,
        })
    }
}

pub struct GroupIterator {
    pub(crate) client: Client,
    pub(crate) group_names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupData {
    Aircraft(AircraftGroupData),
    Ground(GroundGroupData),
    Static(StaticGroupData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AircraftGroupData {
    #[serde(rename = "groupId")]
    pub id: u64,
    pub communication: bool,
    pub frequency: u16,
    pub hidden: bool,
    pub modulation: i64,
    pub name: String,
    pub route: RouteData,
    pub start_time: u64,
    pub task: TaskKind,
    pub tasks: Value, // TODO
    pub uncontrolled: bool,
    pub units: Vec<UnitData>,
    pub x: f64,
    pub y: f64,
    //    #[serde(rename = "radioSet")]
    //    pub radio_set: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundGroupData {
    #[serde(rename = "groupId")]
    pub id: u64,
    pub hidden: bool,
    pub name: String,
    pub route: RouteData,
    pub start_time: u64,
    // while task is set for vehicles, it is only set to "Ground Nothing", so we will ignore it here
    // to have one struct that works for both vehicles and ships
    // pub task: TaskKind,
    pub tasks: Value, // TODO
    pub uncontrollable: bool,
    pub units: Vec<UnitData>,
    pub visible: bool,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticGroupData {
    #[serde(rename = "groupId")]
    pub id: u64,
    pub name: String,
    pub route: RouteData,
    pub units: Vec<UnitData>,
    pub heading: u64,
    #[serde(rename = "linkOffset")]
    pub link_offset: bool,
    pub dead: bool,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteData {
    pub points: Vec<PointData>,
}

// known unimplemented properties: airdromeId, helipadId, formation_template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointData {
    #[serde(rename = "ETA")]
    pub eta: f64,
    #[serde(rename = "ETA_locked")]
    pub eta_locked: bool,
    pub action: WaypointAction,
    pub alt: f64,
    pub alt_type: AltitudeType,
    pub name: String,
    pub properties: Option<Value>,
    pub speed: f64,
    pub speed_locked: bool,
    pub task: Task,
    #[serde(rename = "type")]
    pub kind: WaypointType,
    pub x: f64,
    pub y: f64,
    // TODO: linkUnit for statics
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "id")]
pub enum Task {
    ComboTask {
        params: ComboTaskParams,
    },
    EngageTargets {
        key: EngageTargetsKind,
        enabled: bool,
        number: usize,
        auto: bool,
        params: EngageTargetsParams,
    },
    AttackGroup {
        enabled: bool,
        number: usize,
        auto: bool,
        params: AttackGroupParams,
    },
    AttackUnit {
        enabled: bool,
        number: usize,
        auto: bool,
        params: AttackUnitParams,
    },
    // e.g. RTB
    WrappedAction {
        enabled: bool,
        number: usize,
        auto: bool,
        params: WrappedActionParams,
    },
    FAC {
        enabled: bool,
        number: usize,
        auto: bool,
        params: Value, // TODO
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComboTaskParams {
    tasks: Vec<Task>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngageTargetsParams {
    #[serde(rename = "targetTypes")]
    target_types: Vec<TargetType>,
    priority: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackGroupParams {
    #[serde(rename = "weaponType")]
    weapon_type: usize, // TODO: flags?
    #[serde(rename = "groupId")]
    group_id: usize, // TODO: directly provide Group type
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackUnitParams {
    altitude_enabled: bool,
    unit_id: usize,
    attack_qty_limit: bool,
    attack_qty: usize,
    expend: String, // TODO: enum
    altitude: f64,
    direction_enabled: bool,
    group_attack: bool,
    weapon_type: usize, // TODO: flags?
    direction: usize,   // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WrappedActionParams {
    action: Value, // TODO
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EngageTargetsKind {
    AntiShip,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TargetType {
    Ships,
}

// known unimplemented properties: AddPropAircraft, Radio, hardpoint_racks, livery_id,
// onboard_num, psi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitData {
    #[serde(rename = "unitId")]
    pub id: u64,
    #[serde(rename = "type")]
    pub kind: String, // TODO: enum?
    pub name: String,
    #[serde(default)]
    pub alt: f64,
    #[serde(default)]
    pub alt_type: AltitudeType,
    // statics do not have a callsign
    pub callsign: Option<Value>, // TODO: propper struct
    #[serde(default)]
    pub heading: f64,
    pub payload: Option<Value>, // TODO
    pub skill: Option<Skill>,
    #[serde(default)]
    pub speed: f64,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[serde(rename = "Off Road")]
    OffRoad,
}

impl Default for WaypointAction {
    fn default() -> Self {
        WaypointAction::TurningPoint
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskKind {
    #[serde(rename = "Nothing")]
    Nothing,
    #[serde(rename = "AFAC")]
    AFAC,
    #[serde(rename = "Antiship Strike")]
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
    #[serde(rename = "Ground Nothing")]
    GroundNothing,
}

impl Default for TaskKind {
    fn default() -> Self {
        TaskKind::Nothing
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

impl fmt::Debug for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Group {{ id: {} }}", self.id)
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Group {}", self.id)
    }
}
