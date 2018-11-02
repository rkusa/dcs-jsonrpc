use std::fmt;

#[derive(Debug, Clone, PartialEq, Hash, Serialize)]
pub enum Identifier {
    #[serde(rename = "name")]
    Name(String),
    #[serde(rename = "id")]
    ID(usize),
}

impl From<usize> for Identifier {
    fn from(id: usize) -> Self {
        Identifier::ID(id)
    }
}

impl From<String> for Identifier {
    fn from(name: String) -> Self {
        Identifier::Name(name)
    }
}

impl From<&str> for Identifier {
    fn from(name: &str) -> Self {
        Identifier::Name(name.to_string())
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Identifier::ID(id) => write!(f, "{}", id),
            Identifier::Name(ref name) => write!(f, "{}", name),
        }
    }
}

impl<'de> ::serde::Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct Visitor;

        const FIELDS: &'static [&'static str] = &["id", "name"];

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = Identifier;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("`id` and/or `name`")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Identifier, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut id: Option<usize> = None;
                let mut name: Option<String> = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "id" => {
                            if id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                        "name" => {
                            if name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        k => {
                            return Err(serde::de::Error::unknown_field(&k, FIELDS));
                        }
                    }
                }

                if let Some(name) = name {
                    return Ok(Identifier::Name(name));
                }

                if let Some(id) = id {
                    return Ok(Identifier::ID(id));
                }

                Err(serde::de::Error::missing_field("id"))
            }
        }

        // Deserialize the enum from a u64.
        deserializer.deserialize_map(Visitor)
    }
}
