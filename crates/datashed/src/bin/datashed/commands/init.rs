use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::{env, fs, process};

use clap::{Parser, ValueEnum};
use semver::Version;

use crate::prelude::*;

const GITIGNORE: &'static str = "/data\n/tmp\n\n/index.ipc\n";

/// Create a new datashed or re-initialize an existing one
#[derive(Debug, Parser)]
pub(crate) struct Init {
    #[command(flatten)]
    pub(crate) common: CommonArgs,

    /// The name of the datashed.
    #[arg(short, long)]
    name: Option<String>,

    /// The version of the datashed.
    #[arg(long, default_value = "0.1.0")]
    version: Version,

    /// A short blurb about the datashed.
    #[arg(short, long)]
    description: Option<String>,

    /// A list of people or organizations, which are considered as the
    /// authors of the datashed. By default the list is populated with
    /// the git identity (if available).
    #[arg(short, long = "author")]
    authors: Vec<String>,

    /// Initialize the datashed for the given version control system
    /// (VCS).
    #[arg(long, default_value = "git")]
    vcs: Vcs,

    /// Whether to overwrite config with default values or not.
    #[arg(short, long)]
    force: bool,

    /// The location of the new datashed (default ".")
    directory: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, ValueEnum)]
enum Vcs {
    Git,
    None,
}

fn is_inside_git_work_tree(path: &PathBuf) -> bool {
    process::Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .current_dir(path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn git_init(path: &PathBuf) -> bool {
    process::Command::new("git")
        .arg("init")
        .current_dir(path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn git_get_user<P: AsRef<Path>>(path: P) -> Option<String> {
    let mut user = String::new();

    let result = process::Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("user.name")
        .current_dir(&path)
        .stdout(Stdio::piped())
        .output();

    if let Ok(output) = result {
        if let Ok(name) = std::str::from_utf8(&output.stdout) {
            user.push_str(name.trim_end());
        }
    }

    if user.is_empty() {
        return None;
    }

    let result = process::Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("user.email")
        .current_dir(path)
        .stdout(Stdio::piped())
        .output();

    if let Ok(output) = result {
        if let Ok(email) = std::str::from_utf8(&output.stdout) {
            user.push_str(&format!(" <{}>", email.trim_end()));
        }
    }

    Some(user)
}

impl Init {
    pub(crate) fn execute(mut self) -> CommandResult {
        let root_dir = if let Some(directory) = self.directory {
            env::current_dir()?.join(directory)
        } else {
            env::current_dir()?
        };

        let data_dir = root_dir.join(Datashed::DATA_DIR);
        let tmp_dir = root_dir.join(Datashed::TMP_DIR);
        let config = root_dir.join(Datashed::CONFIG);

        if !root_dir.exists() {
            fs::create_dir_all(&root_dir)?;
        }

        if !data_dir.exists() {
            fs::create_dir(&data_dir)?;
        }

        if !tmp_dir.exists() {
            fs::create_dir(&tmp_dir)?;
        }

        if self.vcs == Vcs::Git {
            if !is_inside_git_work_tree(&root_dir)
                && !git_init(&root_dir)
            {
                bail!("Failed to initialize Git repository");
            }

            let gitignore = root_dir.join(".gitignore");
            if !root_dir.join(".gitignore").is_file() {
                fs::write(&gitignore, GITIGNORE)?;
            }
        }

        if !config.exists() || self.force {
            if self.authors.is_empty() {
                if let Some(author) = git_get_user(&root_dir) {
                    if self.common.verbose {
                        eprintln!(
                            "Set author to Git identity '{author}'"
                        );
                    }

                    self.authors.push(author);
                }
            }

            let mut config = Config::create(config)?;
            config.metadata.description = self.description;
            config.metadata.authors = self.authors;
            config.metadata.version = self.version;
            config.metadata.name = self.name.unwrap_or(
                root_dir
                    .file_name()
                    .and_then(OsStr::to_str)
                    .unwrap_or_default()
                    .to_string(),
            );

            config.save()?;
        }

        if !self.common.quiet {
            eprintln!("Initialized datashed in {}", root_dir.display());
        }

        Ok(SUCCESS)
    }
}
