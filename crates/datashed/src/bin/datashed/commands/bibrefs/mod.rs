use std::fmt::{self, Display};
use std::fs;
use std::path::PathBuf;

use ddc::DdcMatcher;
use doi::DoiMatcher;
use isbn::IsbnMatcher;
use ismn::IsmnMatcher;
use isni::IsniMatcher;
use issn::IssnMatcher;
use jel::JelMatcher;
use lcc::LccMatcher;
use msc::MscMatcher;
use orcid::OrcidMatcher;
use udc::UdcMatcher;

use crate::cli::FilterOpts;
use crate::prelude::*;

mod ddc;
mod doi;
mod isbn;
mod ismn;
mod isni;
mod issn;
mod jel;
mod lcc;
mod msc;
mod orcid;
mod udc;

const PBAR_PROCESS: &str = "Processing documents: {human_pos} ({percent}%) | \
        elapsed: {elapsed_precise}{msg}";

#[derive(Debug, PartialEq)]
pub(crate) struct Reference {
    reftype: ReferenceType,
    value: String,
    start: usize,
    end: usize,
}

#[derive(Debug, PartialEq)]
pub(crate) enum ReferenceType {
    Ddc,
    Doi,
    Isbn,
    Ismn,
    Isni,
    Issn,
    Jel,
    Lcc,
    Msc,
    Orcid,
    Udc,
}

impl Display for ReferenceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ddc => write!(f, "ddc"),
            Self::Doi => write!(f, "doi"),
            Self::Isbn => write!(f, "isbn"),
            Self::Ismn => write!(f, "ismn"),
            Self::Isni => write!(f, "isni"),
            Self::Issn => write!(f, "issn"),
            Self::Jel => write!(f, "jel"),
            Self::Lcc => write!(f, "lcc"),
            Self::Msc => write!(f, "msc"),
            Self::Orcid => write!(f, "orcid"),
            Self::Udc => write!(f, "udc"),
        }
    }
}

pub fn reftype_dtype() -> DataType {
    const CODES: [&str; 11] = [
        "isbn", "issn", "ismn", "doi", "orcid", "isni", "jel", "udc",
        "msc", "ddc", "lcc",
    ];

    DataType::from_frozen_categories(
        FrozenCategories::new(CODES).unwrap(),
    )
}

pub(crate) trait Matcher: Sync {
    fn matches(&self, data: &[u8]) -> Vec<Reference>;
}

/// Finds bibliographic identifiers in documents
#[derive(Debug, clap::Parser)]
pub(crate) struct Bibrefs {
    /// Whether to normalize bibliographic references or not.
    #[arg(long)]
    normalize: bool,

    /// Only use those references that can be found in the Crossref
    /// public data file.
    #[arg(long)]
    crossref: Option<PathBuf>,

    /// Only use those references that can be found in the DataCite
    /// public data files
    #[arg(long)]
    datacite: Option<PathBuf>,

    /// Write the result to <filename>. By default output will be
    /// written in CSV format to stdout
    #[arg(short, long, value_name = "filename")]
    output: Option<PathBuf>,

    #[command(flatten, next_help_heading = "Filter options")]
    pub(crate) filter: FilterOpts,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

#[derive(Debug)]
struct Row {
    path: String,
    hash: String,
    reftype: ReferenceType,
    value: String,
    start: usize,
    end: usize,
}

impl Bibrefs {
    pub(crate) fn execute(self) -> CommandResult {
        let datashed = Datashed::discover()?;
        let data_dir = datashed.data_dir();

        let index = read_index(&datashed, &self.filter)?;
        let paths = index.column("path")?.str()?;
        let hashes = index.column("hash")?.str()?;

        let pbar =
            ProgressBarBuilder::new(PBAR_PROCESS, self.common.quiet)
                .len(index.height() as u64)
                .build();

        let matchers: &[Box<dyn Matcher>] = &[
            Box::new(DoiMatcher::new(
                self.normalize,
                self.crossref,
                self.datacite,
            )),
            Box::new(DdcMatcher::new(self.normalize)),
            Box::new(IsbnMatcher::new(self.normalize)),
            Box::new(IsmnMatcher::new(self.normalize)),
            Box::new(IsniMatcher::new(self.normalize)),
            Box::new(IssnMatcher::new(self.normalize)),
            Box::new(JelMatcher::new(self.normalize)),
            Box::new(LccMatcher::new(self.normalize)),
            Box::new(MscMatcher::new(self.normalize)),
            Box::new(OrcidMatcher::new(self.normalize)),
            Box::new(UdcMatcher::new(self.normalize)),
        ];

        let rows = (0..index.height())
            .into_par_iter()
            .progress_with(pbar)
            .flat_map(|idx| {
                let path = paths.get(idx).unwrap();
                let hash = hashes.get(idx).unwrap();
                let data = fs::read(data_dir.join(path)).unwrap();

                matchers
                    .iter()
                    .flat_map(|m| m.matches(&data))
                    .map(|m| Row {
                        path: path.to_string(),
                        hash: hash.to_string(),
                        reftype: m.reftype,
                        value: m.value,
                        start: m.start,
                        end: m.end,
                    })
                    .collect::<Vec<Row>>()
            })
            .collect::<Vec<Row>>();

        let mut paths = vec![];
        let mut hashes = vec![];
        let mut reftypes = vec![];
        let mut values = vec![];
        let mut starts = vec![];
        let mut ends = vec![];

        for row in rows.into_iter() {
            paths.push(row.path);
            hashes.push(row.hash);
            reftypes.push(row.reftype.to_string());
            values.push(row.value);
            starts.push(row.start as u64);
            ends.push(row.end as u64);
        }

        let mut df = DataFrame::new(vec![
            col!("path", paths),
            col!("hash", hashes),
            if is_arrow(&self.output).unwrap_or_default() {
                col!("reftype", reftypes).cast(&reftype_dtype())?
            } else {
                col!("reftype", reftypes)
            },
            col!("value", values),
            col!("start", starts),
            col!("end", ends),
        ])?
        .lazy()
        .sort(["path", "start"], Default::default())
        .collect()?;

        write_df(&mut df, self.output)?;

        Ok(SUCCESS)
    }
}
