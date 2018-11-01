use std::fmt;

use crate::airbase::Airbase;
use crate::jsonrpc::Client;
use crate::unit::Unit;
use crate::weapon::Weapon;

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
pub(crate) struct RawTarget {
    category: ObjectCategory,
    name: String,
}

#[derive(Clone, Deserialize)]
pub(crate) enum RawEvent {
    Shot {
        time: f64,
        initiator: String,
        weapon: String,
    },
    Hit {
        time: f64,
        initiator: String,
        weapon: String,
        target: RawTarget,
    },
    Takeoff {
        time: f64,
        initiator: String,
        place: String,
    },
    Land {
        time: f64,
        initiator: String,
        place: String,
    },
    Crash {
        time: f64,
        initiator: String,
    },
    Ejection {
        time: f64,
        initiator: String,
    },
    Refueling {
        time: f64,
        initiator: String,
    },
    Dead {
        time: f64,
        initiator: String,
    },
    PilotDead {
        time: f64,
        initiator: String,
    },
    BaseCapture {
        time: f64,
        initiator: String,
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
        initiator: String,
    },
    RefuelingStop {
        time: f64,
        initiator: String,
    },
    Birth {
        time: f64,
        initiator: String,
    },
    SystemFailure {
        time: f64,
        initiator: String,
    },
    EngineStartup {
        time: f64,
        initiator: String,
    },
    EngineShutdown {
        time: f64,
        initiator: String,
    },
    PlayerEnterUnit {
        time: f64,
        initiator: String,
    },
    PlayerLeaveUnit {
        time: f64,
        initiator: String,
    },
    PlayerComment {
        time: f64,
        initiator: String,
    },
    ShootingStart {
        time: f64,
        initiator: String,
    },
    ShootingEnd {
        time: f64,
        initiator: String,
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
                weapon: Weapon::new(client, weapon),
            },
            RawEvent::Hit {
                time,
                initiator,
                weapon,
                target,
            } => Event::Hit {
                time,
                initiator: Unit::new(client.clone(), initiator),
                weapon: Weapon::new(client.clone(), weapon),
                target: match target.category {
                    ObjectCategory::Unit => Object::Unit(Unit::new(client.clone(), target.name)),
                    ObjectCategory::Weapon => {
                        Object::Weapon(Weapon::new(client.clone(), target.name))
                    }
                    ObjectCategory::Base => Object::Base(Airbase::new(client.clone(), target.name)),
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
            } => write!(f, "[{}] Weapon {} fired by {}", time, weapon, initiator),
            Hit {
                time,
                initiator,
                weapon,
                target,
            } => write!(
                f,
                "[{}] Unit {} hit {} with {}",
                time, initiator, target, weapon
            ),
            Takeoff {
                time,
                initiator,
                place,
            } => write!(f, "[{}] Unit {} took off from {}", time, initiator, place),
            Land {
                time,
                initiator,
                place,
            } => write!(f, "[{}] Unit {} landed at {}", time, initiator, place),
            Crash { time, initiator } => write!(f, "[{}] Unit {} crashed", time, initiator),
            Ejection { time, initiator } => write!(f, "[{}] Unit {} ejected", time, initiator),
            Refueling { time, initiator } => {
                write!(f, "[{}] Unit {} started refueling", time, initiator)
            }
            Dead { time, initiator } => write!(f, "[{}] Unit {} died", time, initiator),
            PilotDead { time, initiator } => {
                write!(f, "[{}] Pilot of Unit {} died", time, initiator)
            }
            BaseCapture {
                time,
                initiator,
                place,
            } => write!(f, "[{}] Unit {} captured {}", time, initiator, place),
            MissionStart { time } => write!(f, "[{}] Mission started", time),
            MissionEnd { time } => write!(f, "[{}] Mission ended", time),
            TakeControl { time, initiator } => {
                write!(f, "[{}] A player took control of {}", time, initiator)
            }
            RefuelingStop { time, initiator } => {
                write!(f, "[{}] Unit {} stopped refueling", time, initiator)
            }
            Birth { time, initiator } => write!(f, "[{}] Unit {} was born", time, initiator),
            SystemFailure { time, initiator } => write!(
                f,
                "[{}] Human-controlled unit {} has a system failure",
                time, initiator
            ),
            EngineStartup { time, initiator } => {
                write!(f, "[{}] Unit {} started its engine", time, initiator)
            }
            EngineShutdown { time, initiator } => {
                write!(f, "[{}] Unit {} sthud down its engine", time, initiator)
            }
            PlayerEnterUnit { time, initiator } => {
                write!(f, "[{}] A player entered unit {}", time, initiator)
            }
            PlayerLeaveUnit { time, initiator } => {
                write!(f, "[{}] A player left unit {}", time, initiator)
            }
            PlayerComment { time, initiator } => {
                write!(f, "[{}] The player of {} commented", time, initiator)
            }
            ShootingStart { time, initiator } => {
                write!(f, "[{}] Unit {} started shooting", time, initiator)
            }
            ShootingEnd { time, initiator } => {
                write!(f, "[{}] Unit {} stopped shooting", time, initiator)
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
