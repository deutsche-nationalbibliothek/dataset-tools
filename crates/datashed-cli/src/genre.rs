use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use crate::refinement::Refinements;

pub type GenreRefinements = Refinements<Genre>;

pub(crate) const GENRES: [&str; 4] = [
    "belles-lettres",
    "childrens-and-youth",
    "nonfiction",
    "none",
];

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Genre {
    BellesLettres,
    ChildrensAndYouth,
    Nonfiction,
    #[default]
    None,
}

impl Display for Genre {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BellesLettres => write!(f, "belles-lettres"),
            Self::ChildrensAndYouth => write!(f, "childrens-and-youth"),
            Self::Nonfiction => write!(f, "nonfiction"),
            Self::None => write!(f, "none"),
        }
    }
}
