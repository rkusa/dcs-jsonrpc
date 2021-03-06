use std::fmt;

use crate::jsonrpc::Client;
use crate::{Airbase, Coalition, Error, Position, Scenery, Static, Unit, Weapon};
use serde_json::Value;

#[derive(Debug, Clone)]
pub enum Object {
    Unit(Unit),
    Weapon(Weapon),
    Static(Static),
    Scenery(Scenery),
    Base(Airbase),
    Cargo(Static),
}

#[derive(Debug, Clone)]
pub enum Event<C = usize>
where
    for<'de> C: serde::Serialize + serde::Deserialize<'de>,
{
    /// Occurs when a unit fires a weapon (but no machine gun- or autocannon-based weapons - those
    /// are handled by [Event::ShootingStart]).
    Shot {
        /// The event's mission time.
        time: f64,
        /// The unit that fired the weapon.
        initiator: Unit,
        /// The weapon that has been fired.
        weapon: Weapon,
    },

    /// Occurs when an object is hit by a weapon.
    Hit {
        /// The event's mission time.
        time: f64,
        /// The unit that fired the weapon.
        initiator: Unit,
        /// The weapon that the target has been hit with.
        weapon: Option<Weapon>,
        /// The object that has been hit.
        target: Object,
    },

    /// Occurs when an aircraft takes off from an airbase, farp, or ship.
    Takeoff {
        /// The event's mission time.
        time: f64,
        /// The unit that took off.
        initiator: Unit,
        /// The airbase, farp or ship the unit took off from.
        place: Option<Airbase>,
    },

    /// Occurs when an aircraft lands at an airbase, farp or ship.
    Land {
        /// The event's mission time.
        time: f64,
        /// The unit that landed.
        initiator: Unit,
        /// The airbase, farp or ship the unit landed at.
        place: Option<Airbase>,
    },

    /// Occurs when an aircraft crashes into the ground and is completely destroyed.
    Crash {
        /// The event's mission time.
        time: f64,
        /// The unit that crashed.
        initiator: Unit,
    },

    /// Occurs when a pilot ejects from its aircraft.
    Ejection {
        /// The event's mission time.
        time: f64,
        /// The unit a pilot ejected from.
        initiator: Unit,
    },

    /// Occurs when an aircraft connects with a tanker and begins taking on fuel.
    Refueling {
        /// The event's mission time.
        time: f64,
        /// The unit that is receiving fuel.
        initiator: Unit,
    },

    /// Occurs when an aircraft is finished taking fuel.
    RefuelingStop {
        /// The event's mission time.
        time: f64,
        /// he unit that was receiving fuel.
        initiator: Unit,
    },

    /// Occurs when an object is completely destroyed.
    Dead {
        /// The event's mission time.
        time: f64,
        /// The unit that has been destroyed.
        initiator: Unit,
    },

    /// Occurs when a pilot of an aircraft is killed. Can occur either if the player is alive and
    /// crashes (in this case both this and the [Event::Crash] event will be fired) or if a weapon
    /// kills the pilot without completely destroying the plane.
    PilotDead {
        /// The event's mission time.
        time: f64,
        /// The unit the pilot has died in.
        initiator: Unit,
    },

    /// Occurs when a ground unit captures either an airbase or a farp.
    BaseCapture {
        /// The event's mission time.
        time: f64,
        /// The unit that captured the base.
        initiator: Option<Unit>,
        /// The airbase that was captured, can be a FARP or Airbase
        place: Airbase,
    },

    /// Occurs when the mission starts.
    MissionStart {
        /// The event's mission time.
        time: f64,
    },

    /// Occurs when the mission stops.
    MissionEnd {
        /// The event's mission time.
        time: f64,
    },

    /// Occurs when any object is spawned into the mission.
    Birth {
        /// The event's mission time.
        /// Note: For the birth event, time will always be 0 (might be a DCS bug)
        time: f64,
        /// The unit that was spawned.
        initiator: Unit,
    },

    /// Occurs when a system fails on a human controlled aircraft occurs.
    SystemFailure {
        /// The event's mission time.
        time: f64,
        /// The unit the system failure occurred in.
        initiator: Unit,
    },

    /// Occurs when any aircraft starts its engines.
    EngineStartup {
        /// The event's mission time.
        time: f64,
        /// The unit that starts its engines.
        initiator: Unit,
    },

    /// Occurs when any aircraft shuts down its engines.
    EngineShutdown {
        /// The event's mission time.
        time: f64,
        /// The unit that shuts down its engines.
        initiator: Unit,
    },

    /// Occurs when a player takes direct control of a unit.
    PlayerEnterUnit {
        /// The event's mission time.
        time: f64,
        /// The unit the player took control of.
        initiator: Unit,
    },

    // Occurs when a player relieves direct control of a unit.
    PlayerLeaveUnit {
        /// The event's mission time.
        time: f64,
        /// The unit the player relieves control of.
        initiator: Unit,
    },

    /// Occurs when a unit begins firing a machine gun- or autocannon-based weapon (weapons with a
    /// high rate of fire). Other weapons are handled by [Event::Shot].
    ShootingStart {
        /// The event's mission time.
        time: f64,
        /// The unit that started firing.
        initiator: Unit,
    },

    /// Occurs when a unit stops firing a machine gun- or autocannon-based weapon. Event will always
    /// correspond with a [Event::ShootingStart] event.
    ShootingEnd {
        /// The event's mission time.
        time: f64,
        /// The unit that was shooting and has no stopped firing.
        initiator: Unit,
    },

    /// Occurs when marks get added to the mission by players or scripting functions.
    MarkAdd {
        /// The event's mission time.
        time: f64,
        /// The group the mark's visibility is restricted for.
        group_id: Option<u64>,
        /// The coalition the mark's visibility is restricted for.
        coalition: Option<Coalition>,
        /// The mark's id.
        id: usize,
        /// The unit that added the mark.
        initiator: Option<Unit>,
        /// The position the mark has been added at.
        pos: Position,
        /// The mark's label.
        text: String,
    },

    /// Occurs when marks get edited.
    MarkChange {
        /// The event's mission time.
        time: f64,
        /// The group the mark's visibility is restricted for.
        group_id: Option<u64>,
        /// The coalition the mark's visibility is restricted for.
        coalition: Option<Coalition>,
        /// The mark's id.
        id: usize,
        /// The unit that added the mark.
        initiator: Option<Unit>,
        /// The position the mark has been added at.
        pos: Position,
        /// The mark's label.
        text: String,
    },

    /// Occurs when marks get removed.
    MarkRemove {
        /// The event's mission time.
        time: f64,
        /// The group the mark's visibility is restricted for.
        group_id: Option<u64>,
        /// The coalition the mark's visibility is restricted for.
        coalition: Option<Coalition>,
        /// The mark's id.
        id: usize,
        /// The unit that added the mark.
        initiator: Option<Unit>,
        /// The position the mark has been added at.
        pos: Position,
        /// The mark's label.
        text: String,
    },

    /// Occurs when a player selects an F10-menu command.
    CommandSelect {
        /// The event's mission time.
        time: f64,
        /// The command that has been selected
        command: C,
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
    name: String,
}

#[derive(Clone, Deserialize)]
pub(crate) enum RawEvent {
    Shot {
        time: f64,
        initiator: String,
        weapon: ID,
    },

    Hit {
        time: f64,
        initiator: String,
        weapon: Option<ID>,
        target: RawTarget,
    },

    Takeoff {
        time: f64,
        initiator: String,
        place: Option<String>,
    },

    Land {
        time: f64,
        initiator: String,
        place: Option<String>,
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
        // TODO: might be a weapon (id instead of name)
        initiator: String,
    },

    PilotDead {
        time: f64,
        initiator: String,
    },

    BaseCapture {
        time: f64,
        initiator: Option<String>,
        place: String,
    },

    MissionStart {
        time: f64,
    },

    MissionEnd {
        time: f64,
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

    ShootingStart {
        time: f64,
        initiator: String,
    },

    ShootingEnd {
        time: f64,
        initiator: String,
    },

    MarkAdd {
        time: f64,
        #[serde(rename = "groupId")]
        group_id: Option<u64>,
        coalition: Option<Coalition>,
        id: usize,
        initiator: Option<String>,
        pos: Position,
        text: String,
    },

    MarkChange {
        time: f64,
        #[serde(rename = "groupId")]
        group_id: Option<u64>,
        coalition: Option<Coalition>,
        id: usize,
        initiator: Option<String>,
        pos: Position,
        text: String,
    },

    MarkRemove {
        time: f64,
        #[serde(rename = "groupId")]
        group_id: Option<u64>,
        coalition: Option<Coalition>,
        id: usize,
        initiator: Option<String>,
        pos: Position,
        text: String,
    },

    CommandSelect {
        time: f64,
        command: Value,
    },
}

impl RawEvent {
    pub fn into_event<C>(self, client: Client) -> Result<Event<C>, Error>
    where
        for<'de> C: serde::Serialize + serde::Deserialize<'de>,
    {
        let ev = match self {
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
                weapon: weapon.map(|w| Weapon::new(client.clone(), w.id)),
                target: {
                    match target.category {
                        ObjectCategory::Unit => {
                            Object::Unit(Unit::new(client.clone(), target.name))
                        }
                        ObjectCategory::Weapon => {
                            Object::Weapon(Weapon::new(client.clone(), target.id))
                        }
                        ObjectCategory::Static => {
                            Object::Static(Static::new(client.clone(), target.name))
                        }
                        ObjectCategory::Scenery => {
                            Object::Scenery(Scenery::new(client.clone(), target.name))
                        }
                        ObjectCategory::Base => {
                            Object::Base(Airbase::new(client.clone(), target.name))
                        }
                        ObjectCategory::Cargo => {
                            Object::Cargo(Static::new(client.clone(), target.name))
                        }
                    }
                },
            },
            RawEvent::Takeoff {
                time,
                initiator,
                place,
            } => Event::Takeoff {
                time,
                initiator: Unit::new(client.clone(), initiator),
                place: place.map(|place| Airbase::new(client.clone(), place)),
            },
            RawEvent::Land {
                time,
                initiator,
                place,
            } => Event::Land {
                time,
                initiator: Unit::new(client.clone(), initiator),
                place: place.map(|place| Airbase::new(client.clone(), place)),
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
            RawEvent::RefuelingStop { time, initiator } => Event::RefuelingStop {
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
                initiator: initiator.map(|initiator| Unit::new(client.clone(), initiator)),
                place: Airbase::new(client.clone(), place),
            },
            RawEvent::MissionStart { time } => Event::MissionStart { time },
            RawEvent::MissionEnd { time } => Event::MissionEnd { time },
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
            RawEvent::ShootingStart { time, initiator } => Event::ShootingStart {
                time,
                initiator: Unit::new(client.clone(), initiator),
            },
            RawEvent::ShootingEnd { time, initiator } => Event::ShootingEnd {
                time,
                initiator: Unit::new(client.clone(), initiator),
            },
            RawEvent::MarkAdd {
                time,
                group_id,
                coalition,
                id,
                initiator,
                pos,
                text,
            } => Event::MarkAdd {
                time,
                group_id,
                coalition,
                id,
                initiator: initiator.map(|id| Unit::new(client.clone(), id)),
                pos,
                text,
            },
            RawEvent::MarkChange {
                time,
                group_id,
                coalition,
                id,
                initiator,
                pos,
                text,
            } => Event::MarkChange {
                time,
                group_id,
                coalition,
                id,
                initiator: initiator.map(|id| Unit::new(client.clone(), id)),
                pos,
                text,
            },
            RawEvent::MarkRemove {
                time,
                group_id,
                coalition,
                id,
                initiator,
                pos,
                text,
            } => Event::MarkRemove {
                time,
                group_id,
                coalition,
                id,
                initiator: initiator.map(|id| Unit::new(client.clone(), id)),
                pos,
                text,
            },
            RawEvent::CommandSelect { time, command } => Event::CommandSelect {
                time,
                command: serde_json::from_value(command)?,
            },
        };
        Ok(ev)
    }
}

impl<C> fmt::Display for Event<C>
where
    for<'de> C: serde::Serialize + serde::Deserialize<'de> + fmt::Display,
{
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
            } => write!(
                f,
                "[{}] {} hit {} with {}",
                time,
                initiator,
                target,
                weapon.as_ref().map(|a| a.id()).unwrap_or(0)
            ),
            Takeoff {
                time,
                initiator,
                place,
            } => write!(
                f,
                "[{}] {} took off from {}",
                time,
                initiator,
                place.as_ref().map(|a| a.name()).unwrap_or("ground")
            ),
            Land {
                time,
                initiator,
                place,
            } => write!(
                f,
                "[{}] {} landed at {}",
                time,
                initiator,
                place.as_ref().map(|a| a.name()).unwrap_or("ground")
            ),
            Crash { time, initiator } => write!(f, "[{}] {} crashed", time, initiator),
            Ejection { time, initiator } => write!(f, "[{}] {} ejected", time, initiator),
            Refueling { time, initiator } => {
                write!(f, "[{}] {} started refueling", time, initiator)
            }
            RefuelingStop { time, initiator } => {
                write!(f, "[{}] {} stopped refueling", time, initiator)
            }
            Dead { time, initiator } => write!(f, "[{}] {} died", time, initiator),
            PilotDead { time, initiator } => write!(f, "[{}] Pilot of {} died", time, initiator),
            BaseCapture {
                time,
                initiator,
                place,
            } => write!(
                f,
                "[{}] {} captured {}",
                time,
                initiator
                    .as_ref()
                    .map(|i| i.to_string())
                    .unwrap_or_else(|| String::from("")),
                place
            ),
            MissionStart { time } => write!(f, "[{}] Mission started", time),
            MissionEnd { time } => write!(f, "[{}] Mission ended", time),
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
            ShootingStart { time, initiator } => {
                write!(f, "[{}] {} started shooting", time, initiator)
            }
            ShootingEnd { time, initiator } => {
                write!(f, "[{}] {} stopped shooting", time, initiator)
            }
            MarkAdd {
                time, text, pos, ..
            } => write!(f, "[{}] A mark has been added at {}: {}", time, pos, text),
            MarkChange {
                time, text, pos, ..
            } => write!(f, "[{}] A mark has been changed at {}: {}", time, pos, text),
            MarkRemove {
                time, text, pos, ..
            } => write!(f, "[{}] A mark has been removed at {}: {}", time, pos, text),
            CommandSelect { time, command } => {
                write!(f, "[{}] Menu command {} selected", time, command)
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
            Static(o) => o.fmt(f),
            Scenery(o) => o.fmt(f),
            Base(o) => o.fmt(f),
            Cargo(o) => o.fmt(f),
        }
    }
}
