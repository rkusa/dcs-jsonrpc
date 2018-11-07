use std::fmt;

/// A position in 3D space relative to the map origin.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Position {
    pub x: f64,
    #[serde(rename = "z")]
    pub y: f64,
    #[serde(rename = "y")]
    pub alt: f64,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, alt: {})", self.x, self.y, self.alt)
    }
}
