use std::fmt;

use crate::jsonrpc::Client;

/// Represents all objects placed on the map. Bridges, buildings, etc.
#[derive(Clone, Serialize)]
pub struct Scenery {
    #[serde(skip)]
    client: Client,
    name: String,
}

impl Scenery {
    pub(crate) fn new<N: Into<String>>(client: Client, name: N) -> Self {
        Scenery {
            client,
            name: name.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Debug for Scenery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Scenery {{ name: {} }}", self.name)
    }
}

impl fmt::Display for Scenery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Scenery {}", self.name)
    }
}
