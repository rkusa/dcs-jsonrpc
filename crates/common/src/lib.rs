#[macro_use]
extern crate serde_derive;

use std::{error, fmt};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(untagged)]
pub enum ID {
    Number(i64),
    String(String),
}

#[derive(Debug, Clone, Copy)]
pub enum Version {
    V2,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Request {
    pub jsonrpc: Version,
    pub method: String,
    pub params: Option<Value>,
    pub id: ID,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Notification {
    pub jsonrpc: Version,
    pub method: String,
    pub params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
    pub data: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Success {
        jsonrpc: Version,
        result: Value,
        id: ID,
    },
    Error {
        jsonrpc: Version,
        error: RpcError,
        id: ID,
    },
}

impl Default for ID {
    fn default() -> Self {
        ID::Number(0)
    }
}

impl Default for Version {
    fn default() -> Self {
        Version::V2
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match *self {
            Version::V2 => "2.0",
        })
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "2.0" => Ok(Version::V2),
            _ => Err(::serde::de::Error::custom(format!(
                "unknown {} value: {}",
                stringify!(Version),
                s
            ))),
        }
    }
}

impl fmt::Display for ID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &ID::Number(n) => write!(f, "{}", n),
            &ID::String(ref s) => write!(f, "{}", s),
        }
    }
}

impl fmt::Display for RpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RPC Error: {}", self.message)
    }
}

impl error::Error for RpcError {
    fn description(&self) -> &str {
        "RPC returned error"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}
