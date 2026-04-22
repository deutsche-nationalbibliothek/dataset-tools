use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use crate::refinement::Refinements;

pub type GroupRefinements = Refinements<Group>;

pub(crate) const GROUPS: [&str; 6] = [
    "article",
    "paratext",
    "monograph",
    "collection",
    "other",
    "none",
];

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Group {
    Article,
    Monograph,
    Collection,
    Paratext,
    Other,
    #[default]
    None,
}

impl Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Article => write!(f, "article"),
            Self::Paratext => write!(f, "paratext"),
            Self::Monograph => write!(f, "monograph"),
            Self::Collection => write!(f, "collection"),
            Self::Other => write!(f, "other"),
            Self::None => write!(f, "none"),
        }
    }
}
