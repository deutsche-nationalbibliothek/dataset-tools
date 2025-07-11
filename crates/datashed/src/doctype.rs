use std::fmt::{self, Display};
use std::path::{Component, Path};
use std::str::FromStr;

use hashbrown::HashMap;
use pica_record::path::{Path as PicaPath, PathExt};
use pica_record::prelude::*;
use serde::{Deserialize, Serialize};

pub(crate) const DOCTYPES: [&str; 13] = [
    "blurb",
    "musical-notation",
    "other",
    "statistical-report",
    "bibliography",
    "toc",
    // theses
    "bachelor-thesis",
    "diploma-thesis",
    "doctoral-thesis",
    "master-thesis",
    // Miscellaneous
    "correction",
    "editorial",
    "review",
];

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Doctype {
    Blurb,
    Bibliography,
    Other,
    Toc,

    // Few Text
    MusicalNotation,
    StatisticalReport,

    // Theses
    BachelorThesis,
    DiplomaThesis,
    DoctoralThesis,
    MasterThesis,

    // Miscellaneous
    Correction,
    Editorial,
    Review,
}

impl Default for Doctype {
    fn default() -> Self {
        Self::Other
    }
}

impl FromStr for Doctype {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "blurb" => Self::Blurb,
            "other" => Self::Other,
            "toc" => Self::Toc,
            _ => return Err(()),
        })
    }
}

impl Display for Doctype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blurb => write!(f, "blurb"),
            Self::Bibliography => write!(f, "bibliography"),
            Self::Other => write!(f, "other"),
            Self::Toc => write!(f, "toc"),

            // Few Text
            Self::MusicalNotation => write!(f, "musical-notation"),
            Self::StatisticalReport => write!(f, "statistical-report"),

            // Theses
            Self::BachelorThesis => write!(f, "bachelor-thesis"),
            Self::DiplomaThesis => write!(f, "diploma-thesis"),
            Self::DoctoralThesis => write!(f, "doctoral-thesis"),
            Self::MasterThesis => write!(f, "master-thesis"),

            // Miscellaneous
            Self::Correction => write!(f, "correction"),
            Self::Editorial => write!(f, "editorial"),
            Self::Review => write!(f, "review"),
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

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Refinements {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    refinements: Vec<Refinement>,

    #[serde(flatten)]
    info: RefinementInfo,

    #[serde(skip)]
    map: HashMap<String, Doctype>,
}

impl Refinements {
    pub fn process_record(
        &mut self,
        record: &ByteRecord,
        options: &MatcherOptions,
    ) {
        for refinement in self.refinements.iter_mut() {
            if let Some(doctype) = refinement.is_match(record, options)
            {
                let Some(ppn) = record.ppn() else { return };
                self.map.insert(ppn.to_string(), doctype);
            }
        }
    }

    #[inline]
    pub fn finish(mut self) -> HashMap<String, Doctype> {
        for refinement in self.refinements.iter() {
            let inheritance = refinement.finish();

            for (src, dst) in inheritance.iter() {
                if let Some(doctype) = self.map.get(dst) {
                    self.map.insert(src.to_owned(), doctype.to_owned());
                }
            }
        }

        self.map
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct RefinementInfo {
    description: Option<String>,
    link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Refinement {
    Match(MatchExpr),
    If(IfExpr),
}

impl Refinement {
    fn is_match(
        &mut self,
        record: &ByteRecord,
        options: &MatcherOptions,
    ) -> Option<Doctype> {
        match self {
            Self::Match(expr) => expr.is_match(record, options),
            Self::If(expr) => expr.is_match(record, options),
        }
    }

    pub fn finish(&self) -> &HashMap<String, String> {
        match self {
            Self::Match(expr) => expr.finish(),
            Self::If(expr) => expr.finish(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct MatchExpr {
    #[serde(rename = "match")]
    head: PicaPath,

    #[serde(default)]
    inherit: Option<PicaPath>,

    #[serde(skip)]
    inheritance: HashMap<String, String>,

    #[serde(default)]
    cases: Vec<MatchArm>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
struct MatchArm {
    pattern: MatchPattern,

    #[serde(rename = "if")]
    guard: Option<RecordMatcher>,

    #[serde(rename = "then")]
    doctype: Doctype,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
enum MatchPattern {
    Literal(String),
    List(Vec<String>),
}

impl MatchExpr {
    fn is_match(
        &mut self,
        record: &ByteRecord,
        options: &MatcherOptions,
    ) -> Option<Doctype> {
        if let Some(ref path) = self.inherit {
            if let Some(dst) = record.first(path, options) {
                if let Some(src) = record.ppn() {
                    self.inheritance
                        .insert(src.to_string(), dst.to_string());
                }
            }
        }

        let head = record.first(&self.head, options)?;

        for arm in self.cases.iter() {
            if let Some(ref guard) = arm.guard {
                if !guard.is_match(record, options) {
                    continue;
                }
            }

            let result = match arm.pattern {
                MatchPattern::Literal(ref lit) if lit == "_" => true,
                MatchPattern::Literal(ref lit) => head == lit,
                MatchPattern::List(ref list) => {
                    list.iter().any(|item| item == head)
                }
            };

            if result {
                return Some(arm.doctype);
            }
        }

        None
    }

    pub fn finish(&self) -> &HashMap<String, String> {
        &self.inheritance
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IfExpr {
    #[serde(rename = "if")]
    predicate: RecordMatcher,

    #[serde(default)]
    inherit: Option<PicaPath>,

    #[serde(skip)]
    inheritance: HashMap<String, String>,

    #[serde(rename = "then")]
    doctype: Doctype,
}

impl IfExpr {
    fn is_match(
        &mut self,
        record: &ByteRecord,
        options: &MatcherOptions,
    ) -> Option<Doctype> {
        if let Some(ref path) = self.inherit {
            if let Some(dst) = record.first(path, options) {
                if let Some(src) = record.ppn() {
                    self.inheritance
                        .insert(src.to_string(), dst.to_string());
                }
            }
        }

        if self.predicate.is_match(record, options) {
            Some(self.doctype)
        } else {
            None
        }
    }

    pub fn finish(&self) -> &HashMap<String, String> {
        &self.inheritance
    }
}
