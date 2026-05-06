use std::fs::File;
use std::path::Path;

use serde::{Deserialize, Serialize};
use sophia::api::ns::Namespace;
use sophia::api::prelude::*;
use sophia::api::term::{FromTerm, LanguageTag};
use sophia::inmem::graph::LightGraph;
use sophia::term::{GenericLiteral, RcTerm};
use sophia::turtle::serializer::turtle::{
    TurtleConfig, TurtleSerializer,
};

use crate::DatasetResult;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Vocabulary {
    Listing(VocabularyListing),
}

impl Vocabulary {
    pub fn save(&self, base_dir: &Path) -> DatasetResult<()> {
        match self {
            Self::Listing(vocab) => vocab.save(base_dir),
        }
    }
}

fn default_output() -> String {
    "vocab.ttl".to_string()
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct VocabularyListing {
    #[serde(default = "default_output")]
    output: String,

    base_uri: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    concepts: Vec<Concept>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Concept {
    notation: String,
    labels: Vec<Label>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Label {
    #[serde(default)]
    kind: LabelKind,
    label: String,
    lang: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub enum LabelKind {
    #[default]
    Preferred,
    Alternative,
    Hidden,
}

impl VocabularyListing {
    pub fn save(&self, base_dir: &Path) -> DatasetResult<()> {
        let mut graph = LightGraph::new();

        let base = Namespace::new_unchecked(self.base_uri.clone());
        let rdf = Namespace::new_unchecked(
            "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
        );
        let skos = Namespace::new_unchecked(
            "http://www.w3.org/2004/02/skos/core#",
        );

        for concept in self.concepts.iter() {
            let s = base.get(&concept.notation).unwrap();
            let p = rdf.get("type").unwrap();
            let o = skos.get("Concept").unwrap();

            graph.insert(s, p, o).unwrap();

            for Label { kind, label, lang } in concept.labels.iter() {
                let lang = LanguageTag::new_unchecked(lang.clone());
                let s = base.get(&concept.notation).unwrap();

                let p = match kind {
                    LabelKind::Preferred => skos.get("prefLabel"),
                    LabelKind::Alternative => skos.get("altLabel"),
                    LabelKind::Hidden => skos.get("hiddenLabel"),
                }
                .unwrap();

                let o = RcTerm::from_term(
                    GenericLiteral::LanguageString(label.clone(), lang),
                );

                graph.insert(s, p, o).unwrap();
            }
        }

        let wrt = File::create(base_dir.join(&self.output))?;
        let config = TurtleConfig::new().with_pretty(true);
        let mut ser = TurtleSerializer::new_with_config(wrt, config);
        ser.serialize_graph(&graph).unwrap();

        // println!("{:?}", graph);
        Ok(())
    }
}
