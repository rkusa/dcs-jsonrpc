use std::fmt;

use crate::jsonrpc::Client;

#[derive(Clone)]
pub struct Weapon {
    client: Client,
    id: usize,
}

impl Weapon {
    pub(crate) fn new(client: Client, id: usize) -> Self {
        Weapon { client, id }
    }
}

impl fmt::Display for Weapon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Weapon {}", self.id)
    }
}
