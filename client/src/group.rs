use std::fmt;

use crate::error::Error;
use crate::jsonrpc::Client;

#[derive(Clone)]
pub struct Group {
    client: Client,
    name: String,
}

impl Group {
    pub(crate) fn new<S: Into<String>>(client: Client, name: S) -> Self {
        Group {
            client,
            name: name.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn exists(&self) -> Result<bool, Error> {
        #[derive(Serialize)]
        struct Params<'a> {
            name: &'a str,
        }

        let exists: bool = self
            .client
            .request("groupExists", Some(Params { name: &self.name }))?;

        Ok(exists)
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
