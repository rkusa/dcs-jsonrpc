use std::fmt;

use crate::jsonrpc::Client;
use crate::unit::Unit;
use crate::weapon::Weapon;

#[derive(Clone)]
pub enum Event {
    Shot {
        time: f64,
        initiator: Unit,
        weapon: Weapon,
    },
    MissionEnd {
        time: f64,
    },
    PlayerEnterUnit {
        time: f64,
        initiator: Unit,
    },
}

#[derive(Clone, Deserialize)]
pub(crate) enum RawEvent {
    Shot {
        time: f64,
        initiator: String,
        weapon: String,
    },
    MissionEnd {
        time: f64,
    },
    PlayerEnterUnit {
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
            RawEvent::MissionEnd { time } => Event::MissionEnd { time },
            RawEvent::PlayerEnterUnit { time, initiator } => Event::PlayerEnterUnit {
                time,
                initiator: Unit::new(client, initiator),
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
            MissionEnd { time } => write!(f, "[{}] Mission ended", time),
            PlayerEnterUnit { time, initiator } => {
                write!(f, "[{}] A player entered unit {}", time, initiator)
            }
        }
    }
}
