use std::fmt::{self, Display};
use std::path::{Component, Path};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::refinement::Refinements;

pub type DoctypeRefinements = Refinements<Doctype>;

pub(crate) const DOCTYPES: [&str; 24] = [
    "abstract",
    "blog-post",
    "blurb",
    "conference-object",
    "conference-paper",
    "conference-proceedings",
    "correction",
    "doctoral-thesis",
    "journal-article",
    "journal-object",
    "letter-to-the-editor",
    "musical-notation",
    "other",
    "policy-paper",
    "preface",
    "preprint-article",
    "retraction-note",
    "review",
    "scientific-article",
    "statistical-report",
    "study-thesis",
    "table-of-contents",
    "textbook",
    "working-paper",
];

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum Doctype {
    Abstract,
    BlogPost,
    Blurb,
    ConferenceObject,
    ConferencePaper,
    ConferenceProceedings,
    Correction,
    DoctoralThesis,
    JournalArticle,
    JournalObject,
    LetterToTheEditor,
    MusicalNotation,
    PolicyPaper,
    Preface,
    PreprintArticle,
    RetractionNote,
    Review,
    ScientificArticle,
    StatisticalReport,
    StudyThesis,
    TableOfContents,
    Textbook,
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
            Self::BlogPost => write!(f, "blog-post"),
            Self::Blurb => write!(f, "blurb"),
            Self::ConferenceObject => write!(f, "conference-object"),
            Self::ConferencePaper => write!(f, "conference-paper"),
            Self::ConferenceProceedings => {
                write!(f, "conference-proceedings")
            }
            Self::Correction => write!(f, "correction"),
            Self::DoctoralThesis => write!(f, "doctoral-thesis"),
            Self::JournalArticle => write!(f, "journal-article"),
            Self::JournalObject => write!(f, "journal-object"),
            Self::LetterToTheEditor => {
                write!(f, "letter-to-the-editor")
            }
            Self::MusicalNotation => write!(f, "musical-notation"),
            Self::PolicyPaper => write!(f, "policy-paper"),
            Self::Preface => write!(f, "preface"),
            Self::PreprintArticle => write!(f, "preprint-article"),
            Self::RetractionNote => write!(f, "retraction-note"),
            Self::Review => write!(f, "review"),
            Self::ScientificArticle => write!(f, "scientific-article"),
            Self::StatisticalReport => write!(f, "statistical-report"),
            Self::StudyThesis => write!(f, "study-thesis"),
            Self::TableOfContents => write!(f, "table-of-contents"),
            Self::Textbook => write!(f, "textbook"),
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
