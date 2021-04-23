use std::fs;

use anyhow::Result;
use clap::ArgMatches;
use thiserror::Error;

use prs_lib::{Secret, Store};

use crate::cmd::matcher::{duplicate::DuplicateMatcher, MainMatcher, Matcher};
use crate::util::{cli, error, select, sync};

/// Duplicate secret action.
pub struct Duplicate<'a> {
    cmd_matches: &'a ArgMatches,
}

impl<'a> Duplicate<'a> {
    /// Construct a new duplicate action.
    pub fn new(cmd_matches: &'a ArgMatches) -> Self {
        Self { cmd_matches }
    }

    /// Invoke the duplicate action.
    pub fn invoke(&self) -> Result<()> {
        // Create the command matchers
        let matcher_main = MainMatcher::with(self.cmd_matches).unwrap();
        let matcher_duplicate = DuplicateMatcher::with(self.cmd_matches).unwrap();

        let store = Store::open(matcher_duplicate.store()).map_err(Err::Store)?;
        let sync = store.sync();

        sync::ensure_ready(&sync);
        sync.prepare()?;

        let secret = select::store_select_secret(&store, matcher_duplicate.query())
            .ok_or(Err::NoneSelected)?;
        let dest = matcher_duplicate.destination();

        // TODO: show secret name if not equal to query, unless quiet?

        // Normalize dest path
        let path = store
            .normalize_secret_path(dest, secret.path.file_name().and_then(|p| p.to_str()), true)
            .map_err(Err::NormalizePath)?;
        let new_secret = Secret::from(&store, path.to_path_buf());

        // Check if destination already exists if not forcing
        if !matcher_main.force() && path.is_file() {
            eprintln!("A secret at '{}' already exists", path.display(),);
            if !cli::prompt_yes("Overwrite?", Some(true), &matcher_main) {
                if matcher_main.verbose() {
                    eprintln!("Duplication cancelled");
                }
                error::quit();
            }
        }

        // Copy secret
        fs::copy(&secret.path, path).map_err(Err::Copy)?;

        sync.finalize(format!(
            "Duplicate from {} to {}",
            secret.name, new_secret.name
        ))?;

        if !matcher_main.quiet() {
            eprintln!("Secret duplicated");
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Err {
    #[error("failed to access password store")]
    Store(#[source] anyhow::Error),

    #[error("no secret selected")]
    NoneSelected,

    #[error("failed to normalize destination path")]
    NormalizePath(#[source] anyhow::Error),

    #[error("failed to copy secret file")]
    Copy(#[source] std::io::Error),
}
