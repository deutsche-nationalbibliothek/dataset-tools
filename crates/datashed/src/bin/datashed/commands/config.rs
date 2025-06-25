use clap::Parser;
use datashed::{NormalizationForm, Runtime};

use crate::prelude::*;

/// Get and set datashed config options.
#[derive(Debug, Parser)]
pub(crate) struct Config {
    /// Get the value for the given key
    #[arg(long, conflicts_with_all = ["value", "unset", "set"])]
    get: bool,

    /// Remove the key from the config
    #[arg(long, conflicts_with_all = ["value", "get", "set"])]
    unset: bool,

    /// Set the value for the given key
    #[arg(long, requires = "value", conflicts_with_all = ["get", "unset"])]
    set: bool,

    /// The name of the config option
    name: String,

    /// The (new) value of the config option
    #[arg(conflicts_with_all = ["get", "unset"])]
    value: Option<String>,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

macro_rules! set_rt_option {
    ($config:expr, $name:ident, $value:expr, $type:ty) => {
        if let Ok(value) = $value.parse::<$type>() {
            if let Some(ref mut runtime) = $config.runtime {
                runtime.$name = Some(value);
            } else {
                $config.runtime = Some(Runtime {
                    $name: Some(value),
                    ..Default::default()
                });
            }

            $config.save()?;
        } else {
            bail!("invalid value `{}`", $value);
        }
    };
}

macro_rules! unset_rt_option {
    ($config:expr, $option:ident) => {
        if let Some(ref mut runtime) = $config.runtime {
            runtime.$option = None;
            $config.save()?;
        }
    };
}

macro_rules! print_option {
    ($name:expr, $value:expr) => {
        let value = match $value {
            Some(value) => value.to_string(),
            None => "None".to_string(),
        };

        println!("{} = {}", $name, value);
    };
}

impl Config {
    pub(crate) fn execute(self) -> CommandResult {
        let datashed = Datashed::discover()?;
        let mut config = datashed.config()?;

        let name = match self.name.as_str() {
            name if name == "runtime.num-jobs" => name,
            name if name == "runtime.normalization" => name,
            name => {
                bail!("unknown config option `{name}`");
            }
        };

        if self.value.is_some() {
            let value = self.value.unwrap();
            match name {
                "runtime.num-jobs" => {
                    set_rt_option!(config, num_jobs, value, usize);
                }
                "runtime.normalization" => {
                    set_rt_option!(
                        config,
                        normalization,
                        value,
                        NormalizationForm
                    );
                }
                _ => unreachable!(),
            }
        } else if self.unset {
            match name {
                "runtime.num-jobs" => {
                    unset_rt_option!(config, num_jobs);
                }
                "runtime.normalization" => {
                    unset_rt_option!(config, normalization);
                }
                _ => unreachable!(),
            }
        } else if self.get || (!self.unset && !self.set) {
            match name {
                "runtime.num-jobs" => {
                    print_option!(
                        name,
                        config.runtime.and_then(|rt| rt.num_jobs)
                    );
                }
                "runtime.normalization" => {
                    print_option!(
                        name,
                        config.runtime.and_then(|rt| rt.normalization)
                    );
                }
                _ => unreachable!(),
            }
        } else {
            unreachable!()
        }

        Ok(SUCCESS)
    }
}
