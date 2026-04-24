use anyhow::bail;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::DatasetResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct Remote {
    pub(crate) url: Url,
    pub(crate) predicate: Option<String>,
}

impl Remote {
    pub fn new<U: Into<Url>, S: ToString>(
        url: U,
        query: Option<S>,
    ) -> DatasetResult<Self> {
        let url = url.into();
        let scheme = url.scheme();

        if scheme != "https" {
            bail!("unsupported scheme {scheme}");
        }

        Ok(Self {
            url,
            predicate: query.map(|s| s.to_string()),
        })
    }

    pub fn set_url<U: Into<Url>>(
        &mut self,
        url: U,
    ) -> DatasetResult<()> {
        let url = url.into();
        let scheme = url.scheme();

        if scheme != "https" {
            bail!("unsupported scheme {scheme}");
        }

        self.url = url;

        Ok(())
    }

    pub fn set_predicate<S: ToString>(&mut self, predicate: S) {
        self.predicate = Some(predicate.to_string());
    }
}
