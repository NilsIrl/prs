use clap::{Arg, ArgMatches};

use super::{CmdArg, CmdArgOption};

/// The store argument.
pub struct ArgStore {}

impl CmdArg for ArgStore {
    fn name() -> &'static str {
        "store"
    }

    fn build<'b, 'c>() -> Arg<'b, 'c> {
        Arg::with_name("store")
            .long("store")
            .short("s")
            .value_name("PATH")
            .global(true)
            .help("Password store to use")
    }
}

impl<'a> CmdArgOption<'a> for ArgStore {
    type Value = String;

    fn value<'b: 'a>(matches: &'a ArgMatches<'b>) -> Self::Value {
        Self::value_raw(matches)
            .unwrap_or(prs_lib::STORE_DEFAULT_ROOT)
            .to_string()
    }
}
