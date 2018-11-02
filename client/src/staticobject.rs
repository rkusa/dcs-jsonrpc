use std::fmt;

use crate::jsonrpc::Client;
use crate::Identifier;

#[derive(Clone)]
pub struct Static {
    client: Client,
    id: Identifier,
}

impl Static {
    pub(crate) fn new<I: Into<Identifier>>(client: Client, id: I) -> Self {
        Static {
            client,
            id: id.into(),
        }
    }
}

impl fmt::Debug for Static {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Static {{ id: {} }}", self.id)
    }
}

impl fmt::Display for Static {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Static {}", self.id)
    }
}
