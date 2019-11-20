use std::fmt;

use crate::jsonrpc::Client;
use crate::{Error, Position};

#[derive(Clone, Serialize)]
pub struct Airbase {
    #[serde(skip)]
    client: Client,
    name: String,
}

impl Airbase {
    pub(crate) fn new<N: Into<String>>(client: Client, name: N) -> Self {
        Airbase {
            client,
            name: name.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    fn request<R>(&self, method: &str) -> Result<R, Error>
    where
        for<'de> R: serde::Deserialize<'de>,
    {
        self.client
            .request::<_, Option<R>>(method, Some(&self))?
            .ok_or_else(|| Error::NonExistent)
    }

    pub fn exists(&self) -> Result<bool, Error> {
        self.client.request("airbaseExists", Some(&self))
    }

    pub fn position(&self) -> Result<Position, Error> {
        self.request("airbasePosition")
    }
}

impl fmt::Debug for Airbase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Airbase {{ name: {} }}", self.name)
    }
}

impl fmt::Display for Airbase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Airbase {}", self.name)
    }
}
