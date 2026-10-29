use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use crate::refinement::Refinements;

pub type GenreRefinements = Refinements<Genre>;

pub(crate) const GENRES: [&str; 4] =
    ["fiction", "childrens-and-youth", "nonfiction", "none"];

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Genre {
    Fiction,
    JuvenileLiterature,
    Nonfiction,
    #[default]
    None,
}

impl Display for Genre {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fiction => write!(f, "fiction"),
            Self::JuvenileLiterature => {
                write!(f, "juvenile-literature")
            }
            Self::Nonfiction => write!(f, "nonfiction"),
            Self::None => write!(f, "none"),
        }
    }
}
