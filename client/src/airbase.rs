use std::fmt;

use crate::jsonrpc::Client;

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
