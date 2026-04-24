use clap::Parser;
use dataset::Runtime;

use crate::prelude::*;

/// Get and set dataset config options.
#[derive(Debug, Parser)]
pub(crate) struct Config {
    /// Get the value for the given key.
    #[arg(long, conflicts_with_all = ["value", "unset", "set"])]
    get: bool,

    /// Remove the key from the config.
    #[arg(long, conflicts_with_all = ["value", "get", "set"])]
    unset: bool,

    /// Set the value for the given key.
    #[arg(long, requires = "value", conflicts_with_all = ["get", "unset"])]
    set: bool,

    /// The name of the config option.
    name: String,

    /// The (new) value of the config option.
    #[arg(conflicts_with_all = ["get", "unset"])]
    value: Option<String>,
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
        let dataset = Dataset::discover()?;
        let mut config = dataset.config()?;

        let name = match self.name.as_str() {
            name if name == "runtime.num-jobs" => name,
            name => {
                bail!("unknown config option `{name}`");
            }
        };

        if let Some(value) = self.value {
            match name {
                "runtime.num-jobs" => {
                    set_rt_option!(config, num_jobs, value, usize);
                }
                _ => unreachable!(),
            }
        } else if self.unset {
            match name {
                "runtime.num-jobs" => {
                    unset_rt_option!(config, num_jobs);
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
                _ => unreachable!(),
            }
        } else {
            unreachable!()
        }

        Ok(SUCCESS)
    }
}
