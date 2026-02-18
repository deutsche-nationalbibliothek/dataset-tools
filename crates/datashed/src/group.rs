use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use crate::refinement::Refinements;

pub type GroupRefinements = Refinements<Group>;

pub(crate) const GROUPS: [&str; 5] =
    ["article", "paratext", "monograph", "collection", "other"];

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Group {
    Article,
    Paratext,
    Monograph,
    Collection,
    #[default]
    Other,
}

impl Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Article => write!(f, "article"),
            Self::Paratext => write!(f, "paratext"),
            Self::Monograph => write!(f, "monograph"),
            Self::Collection => write!(f, "collection"),
            Self::Other => write!(f, "other"),
        }
    }
}
