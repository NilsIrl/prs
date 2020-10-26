use clap::{App, Arg, SubCommand};

use crate::cmd::arg::{ArgQuery, ArgStore, CmdArg};

/// The show command definition.
pub struct CmdShow;

impl CmdShow {
    pub fn build<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("show")
            .alias("s")
            .alias("cat")
            .alias("display")
            .about("Display a secret")
            .arg(
                Arg::with_name("first")
                    .long("first")
                    .help("Show only the first line of the secret"),
            )
            .arg(ArgQuery::build())
            .arg(ArgStore::build())
    }
}
