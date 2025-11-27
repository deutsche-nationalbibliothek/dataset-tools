use bstr::ByteSlice;
use hashbrown::HashMap;
use pica_record::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Refinements<T: Default + ToString + 'static> {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    refinements: Vec<Refinement<T>>,

    #[serde(skip)]
    map: HashMap<String, String>,
}

impl<T: Default + ToString> Refinements<T> {
    pub fn process_record(
        &mut self,
        record: &ByteRecord,
        options: &MatcherOptions,
    ) {
        for refinement in self.refinements.iter_mut() {
            if let Some(output) = refinement.is_match(record, options) {
                let Some(ppn) = record.ppn() else { return };
                self.map.insert(ppn.to_string(), output.to_string());
                break;
            }
        }
    }

    #[inline(always)]
    pub fn finish(self) -> HashMap<String, String> {
        self.map
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Refinement<T: Default + ToString + 'static> {
    Match(MatchExpr<T>),
    If(IfExpr<T>),
}

impl<T: Default + ToString> Refinement<T> {
    pub fn is_match(
        &mut self,
        record: &ByteRecord,
        options: &MatcherOptions,
    ) -> Option<&T> {
        match self {
            Self::Match(expr) => expr.is_match(record, options),
            Self::If(expr) => expr.is_match(record, options),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct IfExpr<T: Default + ToString + 'static> {
    #[serde(rename = "if")]
    predicate: RecordMatcher,

    #[serde(rename = "then")]
    output: T,
}

impl<T: ToString + Default> IfExpr<T> {
    pub fn is_match(
        &mut self,
        record: &ByteRecord,
        options: &MatcherOptions,
    ) -> Option<&T> {
        if self.predicate.is_match(record, options) {
            Some(&self.output)
        } else {
            None
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct MatchExpr<T: Default + ToString + 'static> {
    #[serde(rename = "match")]
    head: Path,

    #[serde(rename = "cases", default)]
    arms: Vec<MatchArm<T>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
struct MatchArm<T: Default + ToString + 'static> {
    pattern: MatchPattern,

    #[serde(rename = "if")]
    guard: Option<RecordMatcher>,

    #[serde(rename = "then")]
    output: T,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, untagged)]
#[serde(rename_all = "kebab-case")]
enum MatchPattern {
    List(Vec<String>),
    Literal(String),
}

impl<T: Default + ToString + 'static> MatchExpr<T> {
    fn is_match(
        &mut self,
        record: &ByteRecord,
        options: &MatcherOptions,
    ) -> Option<&T> {
        let values =
            record.path(&self.head, options).collect::<Vec<_>>();

        for arm in self.arms.iter() {
            if let Some(ref guard) = arm.guard
                && !guard.is_match(record, options)
            {
                continue;
            }

            let result = match arm.pattern {
                MatchPattern::Literal(ref lit) if lit == "_" => true,
                MatchPattern::Literal(ref lit) => {
                    values.contains(&lit.as_bytes().as_bstr())
                }
                MatchPattern::List(ref list) => {
                    list.iter().any(|item| {
                        values.contains(&item.as_bytes().as_bstr())
                    })
                }
            };

            if result {
                return Some(&arm.output);
            }
        }

        None
    }
}
