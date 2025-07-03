use std::fmt::{self, Display};
use std::path::{Component, Path};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

pub(crate) const DOCTYPES: [&str; 3] = ["other", "review", "toc"];

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Doctype {
    Other,
    Review,
    Toc,
}

impl Default for Doctype {
    fn default() -> Self {
        Self::Other
    }
}

impl Display for Doctype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other => write!(f, "other"),
            Self::Review => write!(f, "review"),
            Self::Toc => write!(f, "toc"),
        }
    }
}

impl FromStr for Doctype {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "other" => Self::Other,
            "review" => Self::Review,
            "toc" => Self::Toc,
            _ => return Err(()),
        })
    }
}

impl TryFrom<&Path> for Doctype {
    type Error = ();

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        path.components()
            .filter_map(|component| {
                if let Component::Normal(s) = component {
                    s.to_str()
                } else {
                    None
                }
            })
            .find_map(|s| Doctype::from_str(s).ok())
            .ok_or(())
    }
}
