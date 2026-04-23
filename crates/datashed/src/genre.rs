use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use crate::refinement::Refinements;

pub type GenreRefinements = Refinements<Genre>;

pub(crate) const GENRES: [&str; 5] = [
    "belles-lettres",
    "childrens-and-youth",
    "nonfiction",
    "reference-work",
    "none",
];

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Genre {
    BellesLettres,
    ChildrensAndYouth,
    Nonfiction,
    ReferenceWork,
    #[default]
    None,
}

impl Display for Genre {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BellesLettres => write!(f, "belles-lettres"),
            Self::ChildrensAndYouth => write!(f, "childrens-and-youth"),
            Self::Nonfiction => write!(f, "nonfiction"),
            Self::ReferenceWork => write!(f, "reference-work"),
            Self::None => write!(f, "none"),
        }
    }
}
