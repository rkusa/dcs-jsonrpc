use std::fmt;

use crate::error::Error;
use crate::jsonrpc::Client;

#[derive(Clone)]
pub struct Group {
    client: Client,
    name: String,
}

enum_number!(GroupCategory {
    Airplane = 0,
    Helicopter = 1,
    Ground = 2,
    Ship = 3,
    Train = 4,
});

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

pub struct GroupIterator {
    pub(crate) client: Client,
    pub(crate) group_names: Vec<String>,
}

impl Iterator for GroupIterator {
    type Item = Group;

    fn next(&mut self) -> Option<Self::Item> {
        self.group_names
            .pop()
            .map(|name| Group::new(self.client.clone(), name))
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
