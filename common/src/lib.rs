#[macro_use]
extern crate serde_derive;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ID {
    Number(i64),
    String(String),
}

#[derive(Debug)]
pub enum Version {
    V2,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Request {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
}

#[derive(Debug, Serialize)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
    pub data: Option<Value>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Response {
    Success {
        jsonrpc: String,
        result: Value,
        id: ID,
    },
    Error {
        jsonrpc: String,
        error: RpcError,
        id: ID,
    },
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
