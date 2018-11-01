use std::fmt;

use crate::jsonrpc::Client;

#[derive(Clone)]
pub struct Airbase {
    client: Client,
    name: String,
}

impl Airbase {
    pub(crate) fn new<S: Into<String>>(client: Client, name: S) -> Self {
        Airbase {
            client,
            name: name.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Airbase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
