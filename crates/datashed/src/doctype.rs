use std::fmt::{self, Display};
use std::path::{Component, Path};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::refinement::Refinements;

pub type DoctypeRefinements = Refinements<Doctype>;

pub(crate) const DOCTYPES: [&str; 19] = [
    "abstract",
    "bachelor-thesis",
    "blurb",
    "conference-object",
    "conference-paper",
    "conference-proceedings",
    "correction",
    "diploma-thesis",
    "doctoral-thesis",
    "magister-thesis",
    "master-thesis",
    "policy-paper",
    "postdoctoral-thesis",
    "preface",
    "research-article",
    "review",
    "study-thesis",
    "table-of-contents",
    "working-paper",
    "other",
];

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum Doctype {
    Abstract,
    BachelorThesis,
    Blurb,
    ConferenceObject,
    ConferencePaper,
    ConferenceProceedings,
    Correction,
    DiplomaThesis,
    DoctoralThesis,
    MagisterThesis,
    MasterThesis,
    PolicyPaper,
    PostdoctoralThesis,
    Preface,
    ResearchArticle,
    Review,
    StudyThesis,
    TableOfContents,
    WorkingPaper,
    #[default]
    Other,
}

impl FromStr for Doctype {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "abstract" => Self::Abstract,
            "blurb" => Self::Blurb,
            "other" => Self::Other,
            "toc" => Self::TableOfContents,
            _ => return Err(()),
        })
    }
}

impl Display for Doctype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Abstract => write!(f, "abstract"),
            Self::BachelorThesis => write!(f, "bachelor-thesis"),
            Self::Blurb => write!(f, "blurb"),
            Self::ConferenceObject => write!(f, "conference-object"),
            Self::ConferencePaper => write!(f, "conference-paper"),
            Self::ConferenceProceedings => {
                write!(f, "conference-proceedings")
            }
            Self::Correction => write!(f, "correction"),
            Self::DiplomaThesis => write!(f, "diploma-thesis"),
            Self::DoctoralThesis => write!(f, "doctoral-thesis"),
            Self::MagisterThesis => write!(f, "magister-thesis"),
            Self::MasterThesis => write!(f, "master-thesis"),
            Self::PolicyPaper => write!(f, "policy-paper"),
            Self::PostdoctoralThesis => {
                write!(f, "postdoctoral-thesis")
            }
            Self::Preface => write!(f, "preface"),
            Self::ResearchArticle => write!(f, "research-article")
            Self::Review => write!(f, "review"),
            Self::StudyThesis => write!(f, "study-thesis"),
            Self::TableOfContents => write!(f, "table-of-contents"),
            Self::WorkingPaper => write!(f, "working-paper"),
            Self::Other => write!(f, "other"),
        }
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
