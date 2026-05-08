use std::fmt::{self, Display};
use std::path::{Component, Path};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::refinement::Refinements;

pub type DoctypeRefinements = Refinements<Doctype>;

pub(crate) const DOCTYPES: [&str; 30] = [
    "abstract",
    "bachelor-thesis",
    "blog-post",
    "blurb",
    "conference-object",
    "conference-paper",
    "conference-proceedings",
    "correction",
    "diploma-thesis",
    "doctoral-thesis",
    "journal-article",
    "journal-object",
    "letter-to-the-editor",
    "magister-thesis",
    "master-thesis",
    "musical-notation",
    "nonfiction-book",
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
    // --snip--
    "other",
    "none",
];

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum Doctype {
    Abstract,
    BachelorThesis,
    BlogPost,
    Blurb,
    ConferenceObject,
    ConferencePaper,
    ConferenceProceedings,
    Correction,
    DiplomaThesis,
    DoctoralThesis,
    JournalArticle,
    JournalObject,
    LetterToTheEditor,
    MagisterThesis,
    MasterThesis,
    MusicalNotation,
    #[default]
    None,
    NonfictionBook,
    Other,
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
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Abstract => write!(f, "abstract"),
            Self::BachelorThesis => write!(f, "bachelor-thesis"),
            Self::BlogPost => write!(f, "blog-post"),
            Self::Blurb => write!(f, "blurb"),
            Self::ConferenceObject => write!(f, "conference-object"),
            Self::ConferencePaper => write!(f, "conference-paper"),
            Self::ConferenceProceedings => write!(f, "conference-proceedings"),
            Self::Correction => write!(f, "correction"),
            Self::DiplomaThesis => write!(f, "diploma-thesis"),
            Self::DoctoralThesis => write!(f, "doctoral-thesis"),
            Self::JournalArticle => write!(f, "journal-article"),
            Self::JournalObject => write!(f, "journal-object"),
            Self::LetterToTheEditor => write!(f, "letter-to-the-editor"),
            Self::MagisterThesis => write!(f, "magister-thesis"),
            Self::MasterThesis => write!(f, "master-thesis"),
            Self::MusicalNotation => write!(f, "musical-notation"),
            Self::None => write!(f, "none"),
            Self::NonfictionBook => write!(f, "nonfiction-book"),
            Self::Other => write!(f, "other"),
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
