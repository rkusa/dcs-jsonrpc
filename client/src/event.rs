use std::fmt;

use crate::jsonrpc::Client;
use crate::{Airbase, Identifier, Unit, Weapon};

#[derive(Clone)]
pub enum Object {
    Unit(Unit),
    Weapon(Weapon),
    //    Static(Static),
    //    Scenery(Scenery),
    Base(Airbase),
    //    Cargo(Cargo),
}

#[derive(Clone)]
pub enum Event {
    Shot {
        time: f64,
        initiator: Unit,
        weapon: Weapon,
    },
    Hit {
        time: f64,
        initiator: Unit,
        weapon: Weapon,
        target: Object,
    },
    Takeoff {
        time: f64,
        initiator: Unit,
        place: Airbase,
    },
    Land {
        time: f64,
        initiator: Unit,
        place: Airbase,
    },
    Crash {
        time: f64,
        initiator: Unit,
    },
    Ejection {
        time: f64,
        initiator: Unit,
    },
    Refueling {
        time: f64,
        initiator: Unit,
    },
    Dead {
        time: f64,
        initiator: Unit,
    },
    PilotDead {
        time: f64,
        initiator: Unit,
    },
    BaseCapture {
        time: f64,
        initiator: Unit,
        place: Airbase,
    },
    MissionStart {
        time: f64,
    },
    MissionEnd {
        time: f64,
    },
    TakeControl {
        time: f64,
        initiator: Unit,
    },
    RefuelingStop {
        time: f64,
        initiator: Unit,
    },
    Birth {
        time: f64,
        initiator: Unit,
    },
    SystemFailure {
        time: f64,
        initiator: Unit,
    },
    EngineStartup {
        time: f64,
        initiator: Unit,
    },
    EngineShutdown {
        time: f64,
        initiator: Unit,
    },
    PlayerEnterUnit {
        time: f64,
        initiator: Unit,
    },
    PlayerLeaveUnit {
        time: f64,
        initiator: Unit,
    },
    PlayerComment {
        time: f64,
        initiator: Unit,
    },
    ShootingStart {
        time: f64,
        initiator: Unit,
    },
    ShootingEnd {
        time: f64,
        initiator: Unit,
    },
}

enum_number!(ObjectCategory {
  Unit    = 1,
  Weapon  = 2,
  Static  = 3,
  Scenery = 4,
  Base    = 5,
  Cargo   = 6,
});

#[derive(Clone, Deserialize)]
pub(crate) struct ID {
    id: usize,
}

#[derive(Clone, Deserialize)]
pub(crate) struct RawTarget {
    category: ObjectCategory,
    id: usize,
    name: Option<String>,
}

#[derive(Clone, Deserialize)]
pub(crate) enum RawEvent {
    Shot {
        time: f64,
        initiator: Identifier,
        weapon: ID,
    },
    Hit {
        time: f64,
        initiator: Identifier,
        weapon: ID,
        target: RawTarget,
    },
    Takeoff {
        time: f64,
        initiator: Identifier,
        place: String,
    },
    Land {
        time: f64,
        initiator: Identifier,
        place: String,
    },
    Crash {
        time: f64,
        initiator: Identifier,
    },
    Ejection {
        time: f64,
        initiator: Identifier,
    },
    Refueling {
        time: f64,
        initiator: Identifier,
    },
    Dead {
        time: f64,
        initiator: Identifier,
    },
    PilotDead {
        time: f64,
        initiator: Identifier,
    },
    BaseCapture {
        time: f64,
        initiator: Identifier,
        place: String,
    },
    MissionStart {
        time: f64,
    },
    MissionEnd {
        time: f64,
    },
    TakeControl {
        time: f64,
        initiator: Identifier,
    },
    RefuelingStop {
        time: f64,
        initiator: Identifier,
    },
    Birth {
        time: f64,
        initiator: Identifier,
    },
    SystemFailure {
        time: f64,
        initiator: Identifier,
    },
    EngineStartup {
        time: f64,
        initiator: Identifier,
    },
    EngineShutdown {
        time: f64,
        initiator: Identifier,
    },
    PlayerEnterUnit {
        time: f64,
        initiator: Identifier,
    },
    PlayerLeaveUnit {
        time: f64,
        initiator: Identifier,
    },
    PlayerComment {
        time: f64,
        initiator: Identifier,
    },
    ShootingStart {
        time: f64,
        initiator: Identifier,
    },
    ShootingEnd {
        time: f64,
        initiator: Identifier,
    },
}

impl RawEvent {
    pub fn into_event(self, client: Client) -> Event {
        match self {
            RawEvent::Shot {
                time,
                initiator,
                weapon,
            } => Event::Shot {
                time,
                initiator: Unit::new(client.clone(), initiator),
                weapon: Weapon::new(client, weapon.id),
            },
            RawEvent::Hit {
                time,
                initiator,
                weapon,
                target,
            } => Event::Hit {
                time,
                initiator: Unit::new(client.clone(), initiator),
                weapon: Weapon::new(client.clone(), weapon.id),
                target: match target.category {
                    ObjectCategory::Unit => Object::Unit(Unit::new(client.clone(), target.id)),
                    ObjectCategory::Weapon => {
                        Object::Weapon(Weapon::new(client.clone(), target.id))
                    }
                    ObjectCategory::Base => Object::Base(Airbase::new(client.clone(), target.id)),
                    _ => unimplemented!(), // TODO
                },
            },
            RawEvent::Takeoff {
                time,
                initiator,
                place,
            } => Event::Takeoff {
                time,
                initiator: Unit::new(client.clone(), initiator),
                place: Airbase::new(client.clone(), place),
            },
            RawEvent::Land {
                time,
                initiator,
                place,
            } => Event::Land {
                time,
                initiator: Unit::new(client.clone(), initiator),
                place: Airbase::new(client.clone(), place),
            },
            RawEvent::Crash { time, initiator } => Event::Crash {
                time,
                initiator: Unit::new(client.clone(), initiator),
            },
            RawEvent::Ejection { time, initiator } => Event::Ejection {
                time,
                initiator: Unit::new(client.clone(), initiator),
            },
            RawEvent::Refueling { time, initiator } => Event::Refueling {
                time,
                initiator: Unit::new(client.clone(), initiator),
            },
            RawEvent::Dead { time, initiator } => Event::Dead {
                time,
                initiator: Unit::new(client.clone(), initiator),
            },
            RawEvent::PilotDead { time, initiator } => Event::PilotDead {
                time,
                initiator: Unit::new(client.clone(), initiator),
            },
            RawEvent::BaseCapture {
                time,
                initiator,
                place,
            } => Event::BaseCapture {
                time,
                initiator: Unit::new(client.clone(), initiator),
                place: Airbase::new(client.clone(), place),
            },
            RawEvent::MissionStart { time } => Event::MissionStart { time },
            RawEvent::MissionEnd { time } => Event::MissionEnd { time },
            RawEvent::TakeControl { time, initiator } => Event::TakeControl {
                time,
                initiator: Unit::new(client.clone(), initiator),
            },
            RawEvent::RefuelingStop { time, initiator } => Event::RefuelingStop {
                time,
                initiator: Unit::new(client.clone(), initiator),
            },
            RawEvent::Birth { time, initiator } => Event::Birth {
                time,
                initiator: Unit::new(client.clone(), initiator),
            },
            RawEvent::SystemFailure { time, initiator } => Event::SystemFailure {
                time,
                initiator: Unit::new(client.clone(), initiator),
            },
            RawEvent::EngineStartup { time, initiator } => Event::EngineStartup {
                time,
                initiator: Unit::new(client.clone(), initiator),
            },
            RawEvent::EngineShutdown { time, initiator } => Event::EngineShutdown {
                time,
                initiator: Unit::new(client.clone(), initiator),
            },
            RawEvent::PlayerEnterUnit { time, initiator } => Event::PlayerEnterUnit {
                time,
                initiator: Unit::new(client, initiator),
            },
            RawEvent::PlayerLeaveUnit { time, initiator } => Event::PlayerLeaveUnit {
                time,
                initiator: Unit::new(client.clone(), initiator),
            },
            RawEvent::PlayerComment { time, initiator } => Event::PlayerComment {
                time,
                initiator: Unit::new(client.clone(), initiator),
            },
            RawEvent::ShootingStart { time, initiator } => Event::ShootingStart {
                time,
                initiator: Unit::new(client.clone(), initiator),
            },
            RawEvent::ShootingEnd { time, initiator } => Event::ShootingEnd {
                time,
                initiator: Unit::new(client.clone(), initiator),
            },
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Event::*;

        match self {
            Shot {
                time,
                initiator,
                weapon,
            } => write!(f, "[{}] {} fired by {}", time, weapon, initiator),
            Hit {
                time,
                initiator,
                weapon,
                target,
            } => write!(f, "[{}] {} hit {} with {}", time, initiator, target, weapon),
            Takeoff {
                time,
                initiator,
                place,
            } => write!(f, "[{}] {} took off from {}", time, initiator, place),
            Land {
                time,
                initiator,
                place,
            } => write!(f, "[{}] {} landed at {}", time, initiator, place),
            Crash { time, initiator } => write!(f, "[{}] {} crashed", time, initiator),
            Ejection { time, initiator } => write!(f, "[{}] {} ejected", time, initiator),
            Refueling { time, initiator } => {
                write!(f, "[{}] {} started refueling", time, initiator)
            }
            Dead { time, initiator } => write!(f, "[{}] {} died", time, initiator),
            PilotDead { time, initiator } => write!(f, "[{}] Pilot of {} died", time, initiator),
            BaseCapture {
                time,
                initiator,
                place,
            } => write!(f, "[{}] {} captured {}", time, initiator, place),
            MissionStart { time } => write!(f, "[{}] Mission started", time),
            MissionEnd { time } => write!(f, "[{}] Mission ended", time),
            TakeControl { time, initiator } => {
                write!(f, "[{}] A player took control of {}", time, initiator)
            }
            RefuelingStop { time, initiator } => {
                write!(f, "[{}] {} stopped refueling", time, initiator)
            }
            Birth { time, initiator } => write!(f, "[{}] Unit {} was born", time, initiator),
            SystemFailure { time, initiator } => write!(
                f,
                "[{}] Human-controlled {} has a system failure",
                time, initiator
            ),
            EngineStartup { time, initiator } => {
                write!(f, "[{}] {} started its engine", time, initiator)
            }
            EngineShutdown { time, initiator } => {
                write!(f, "[{}] {} shut down its engine", time, initiator)
            }
            PlayerEnterUnit { time, initiator } => {
                write!(f, "[{}] A player entered {}", time, initiator)
            }
            PlayerLeaveUnit { time, initiator } => {
                write!(f, "[{}] A player left {}", time, initiator)
            }
            PlayerComment { time, initiator } => {
                write!(f, "[{}] The player of {} commented", time, initiator)
            }
            ShootingStart { time, initiator } => {
                write!(f, "[{}] {} started shooting", time, initiator)
            }
            ShootingEnd { time, initiator } => {
                write!(f, "[{}] {} stopped shooting", time, initiator)
            }
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Object::*;

        match self {
            Unit(o) => o.fmt(f),
            Weapon(o) => o.fmt(f),
            Base(o) => o.fmt(f),
        }
    }
}
