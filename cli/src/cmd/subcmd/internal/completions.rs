use clap::{App, Arg, Shell, SubCommand};

/// The generate completions command definition.
pub struct CmdCompletions;

impl CmdCompletions {
    pub fn build<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("completions")
            .about("Shell completions")
            .alias("completion")
            .alias("complete")
            .arg(
                Arg::with_name("SHELL")
                    .help("Shell type to generate completions for")
                    .required(true)
                    .multiple(true)
                    .takes_value(true)
                    .possible_value("all")
                    .possible_values(&Shell::variants())
                    .case_insensitive(true),
            )
            .arg(
                Arg::with_name("output")
                    .long("output")
                    .short("o")
                    .alias("output-dir")
                    .alias("out")
                    .alias("dir")
                    .value_name("DIR")
                    .help("Shell completion files output directory"),
            )
            .arg(
                Arg::with_name("stdout")
                    .long("stdout")
                    .short("s")
                    .alias("print")
                    .help("Output completion files to stdout instead")
                    .conflicts_with("output"),
            )
            .arg(
                Arg::with_name("name")
                    .long("name")
                    .short("n")
                    .alias("bin")
                    .alias("binary")
                    .alias("bin-name")
                    .alias("binary-name")
                    .value_name("NAME")
                    .help("Name of binary to generate completions for"),
            )
    }
}