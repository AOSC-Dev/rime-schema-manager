use clap::{Command, Arg};

/// Build the CLI instance
pub fn build_cli() -> Command {
    Command::new("Rime Schema Manager")
        .version(env!("CARGO_PKG_VERSION"))
        .author("AOSC-Dev")
        .about("Rime Schema Manager")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Add the specified schema to the configuration")
                .arg(
                    Arg::new("INPUT")
                        .help("Sets the input file to use")
                        .num_args(1..),
                ),
        )
        .subcommand(
            Command::new("remove")
                .about("Remove the specified schema from the configuration")
                .arg(Arg::new("INPUT").help("Schema to be removed").num_args(1..)),
        )
        .subcommand(
            Command::new("sync").about("Synchronize the configuration files with the installed schema"),
        )
        .subcommand(
            Command::new("set-default")
                .about("Set the specified schema to be the default schema")
                .arg(
                    Arg::new("INPUT")
                        .help("Schema to be set as the default")
                        .required(true),
                ),
        )
        .subcommand(Command::new("list").about("List installed schema"))
}
