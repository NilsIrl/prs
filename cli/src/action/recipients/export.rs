use std::fs;
use std::io::Write;

use anyhow::Result;
use clap::ArgMatches;
use thiserror::Error;

use prs_lib::{
    crypto::{self, prelude::*},
    Store,
};

use crate::cmd::matcher::{
    recipients::{export::ExportMatcher, RecipientsMatcher},
    MainMatcher, Matcher,
};
#[cfg(feature = "clipboard")]
use crate::util::clipboard;
use crate::util::select;

/// A recipients export action.
pub struct Export<'a> {
    cmd_matches: &'a ArgMatches<'a>,
}

impl<'a> Export<'a> {
    /// Construct a new export action.
    pub fn new(cmd_matches: &'a ArgMatches<'a>) -> Self {
        Self { cmd_matches }
    }

    /// Invoke the export action.
    pub fn invoke(&self) -> Result<()> {
        // Create the command matchers
        let matcher_main = MainMatcher::with(self.cmd_matches).unwrap();
        let matcher_recipients = RecipientsMatcher::with(self.cmd_matches).unwrap();
        let matcher_export = ExportMatcher::with(self.cmd_matches).unwrap();

        let store = Store::open(matcher_recipients.store()).map_err(Err::Store)?;
        let recipients = store.recipients().map_err(Err::Load)?;

        let key = select::select_key(recipients.keys())
            .ok_or(Err::NoneSelected)?
            .clone();

        // Export public key
        let data = crypto::context(crypto::PROTO)?.export_key(key)?;

        let mut stdout = true;

        // Output to file
        if let Some(path) = matcher_export.output_file() {
            stdout = false;
            fs::write(path, &data).map_err(Err::Output)?;
            if !matcher_main.quiet() {
                eprintln!("Key exported to: {}", path);
            }
        }

        // Copy to clipboard
        #[cfg(feature = "clipboard")]
        if matcher_export.copy() {
            stdout = false;
            clipboard::set(&data).map_err(Err::Clipboard)?;
        }

        if stdout {
            std::io::stdout().write_all(&data).map_err(Err::Output)?;
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Err {
    #[error("failed to access password store")]
    Store(#[source] anyhow::Error),

    #[error("failed to load recipients from keychain")]
    Load(#[source] anyhow::Error),

    #[error("no key selected")]
    NoneSelected,

    #[error("failed to write key to file")]
    Output(#[source] std::io::Error),

    #[cfg(feature = "clipboard")]
    #[error("failed to copy key to clipboard")]
    Clipboard(#[source] anyhow::Error),
}
