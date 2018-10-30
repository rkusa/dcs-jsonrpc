use std::fmt;

use crate::jsonrpc::Client;

#[derive(Clone)]
pub struct Unit {
    client: Client,
    name: String,
}

impl Unit {
    pub(crate) fn new<S: Into<String>>(client: Client, name: S) -> Self {
        Unit {
            client,
            name: name.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
