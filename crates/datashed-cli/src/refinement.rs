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

    scope: Option<RecordMatcher>,

    #[serde(skip)]
    map: HashMap<String, String>,
}

impl<T: Default + ToString> Refinements<T> {
    pub fn process_record(
        &mut self,
        record: &ByteRecord,
        options: &MatcherOptions,
    ) {
        if let Some(ref scope) = self.scope
            && !scope.is_match(record, options)
        {
            return;
        }

        for refinement in self.refinements.iter_mut() {
            if let Some(output) = refinement.is_match(record, options) {
                let Some(ppn) = record.ppn() else { return };
                self.map.insert(ppn.to_string(), output.to_string());
                break;
            }
        }
    }

    pub fn finish(mut self) -> HashMap<String, String> {
        for refinement in self.refinements.iter() {
            if let Some(inheritance) = refinement.finish() {
                for (src, dst) in inheritance.iter() {
                    if let Some(doctype) = self.map.get(dst) {
                        // eprintln!("src {src} -> dst {dst}");
                        self.map
                            .insert(src.to_owned(), doctype.to_owned());
                    }
                }
            }
        }

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

    pub fn finish(&self) -> Option<&HashMap<String, String>> {
        match self {
            Self::Match(expr) => expr.finish(),
            Self::If(expr) => expr.finish(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
struct Inherit {
    path: Path,

    #[serde(rename = "if")]
    guard: Option<RecordMatcher>,

    #[serde(skip)]
    map: HashMap<String, String>,
}

impl Inherit {
    pub fn process_record(
        &mut self,
        record: &ByteRecord,
        options: &MatcherOptions,
    ) {
        if let Some(ref guard) = self.guard
            && !guard.is_match(record, options)
        {
            return;
        }

        if let Some(key) = record.first(&self.path, options)
            && let Some(value) = record.ppn()
        {
            self.map.insert(key.to_string(), value.to_string());
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct IfExpr<T: Default + ToString + 'static> {
    description: Option<String>,

    #[serde(rename = "if")]
    predicate: RecordMatcher,

    scope: Option<RecordMatcher>,
    inherit: Option<Inherit>,

    #[serde(rename = "then")]
    output: T,

    #[serde(default)]
    comment: Option<String>,
}

impl<T: ToString + Default> IfExpr<T> {
    pub fn is_match(
        &mut self,
        record: &ByteRecord,
        options: &MatcherOptions,
    ) -> Option<&T> {
        if let Some(ref mut inherit) = self.inherit {
            inherit.process_record(record, options)
        }

        if let Some(ref scope) = self.scope
            && !scope.is_match(record, options)
        {
            return None;
        }

        if self.predicate.is_match(record, options) {
            Some(&self.output)
        } else {
            None
        }
    }

    pub fn finish(&self) -> Option<&HashMap<String, String>> {
        if let Some(ref inherit) = self.inherit {
            Some(&inherit.map)
        } else {
            None
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct MatchExpr<T: Default + ToString + 'static> {
    description: Option<String>,

    #[serde(rename = "match")]
    head: Path,

    scope: Option<RecordMatcher>,

    inherit: Option<Inherit>,

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

    #[serde(default)]
    comment: Option<String>,
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
        if let Some(ref mut inherit) = self.inherit {
            inherit.process_record(record, options)
        }

        if let Some(ref scope) = self.scope
            && !scope.is_match(record, options)
        {
            return None;
        }

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

    pub fn finish(&self) -> Option<&HashMap<String, String>> {
        if let Some(ref inherit) = self.inherit {
            Some(&inherit.map)
        } else {
            None
        }
    }
}
