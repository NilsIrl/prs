use clap::App;

/// The sync init command definition.
pub struct CmdInit;

impl CmdInit {
    pub fn build<'a>() -> App<'a> {
        App::new("init")
            .alias("initialize")
            .about("Initialize sync")
    }
}
