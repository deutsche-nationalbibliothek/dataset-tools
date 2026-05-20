use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use crate::refinement::Refinements;

pub type GroupRefinements = Refinements<Group>;

pub(crate) const GROUPS: [&str; 7] = [
    "article",
    "monograph",
    "collection",
    "periodical",
    "paratext",
    // -- snip ---
    "other",
    "none",
];

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Group {
    Article,
    Monograph,
    Collection,
    Periodical,
    Paratext,
    Other,
    #[default]
    None,
}

impl Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Article => write!(f, "article"),
            Self::Collection => write!(f, "collection"),
            Self::Monograph => write!(f, "monograph"),
            Self::Paratext => write!(f, "paratext"),
            Self::Periodical => write!(f, "periodical"),
            Self::Other => write!(f, "other"),
            Self::None => write!(f, "none"),
        }
    }
}
