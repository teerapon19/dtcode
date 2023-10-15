use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Content {
    Text,
    RemoveEmpty,
}

impl Display for Content {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Content::Text => write!(f, "text"),
            Content::RemoveEmpty => write!(f, "remove_empty"),
        }
    }
}

impl FromStr for Content {
    type Err = ();

    fn from_str(input: &str) -> Result<Content, Self::Err> {
        match input {
            "text" => Ok(Content::Text),
            "remove_empty" => Ok(Content::RemoveEmpty),
            _ => Err(()),
        }
    }
}
