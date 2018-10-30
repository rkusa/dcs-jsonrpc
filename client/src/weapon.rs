use std::fmt;

use crate::jsonrpc::Client;

#[derive(Clone)]
pub struct Weapon {
    client: Client,
    name: String,
}

impl Weapon {
    pub(crate) fn new<S: Into<String>>(client: Client, name: S) -> Self {
        Weapon {
            client,
            name: name.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Weapon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
