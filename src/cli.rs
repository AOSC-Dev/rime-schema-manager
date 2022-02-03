use clap::{App, AppSettings, Arg};

/// Build the CLI instance
pub fn build_cli() -> App<'static> {
    App::new("Rime Schema Manager")
        .version(env!("CARGO_PKG_VERSION"))
        .author("AOSC-Dev")
        .about("Rime Schema Manager")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            App::new("add")
                .about("Add the specified schema to the configuration")
                .arg(
                    Arg::new("INPUT")
                        .help("Sets the input file to use")
                        .min_values(1),
                ),
        )
        .subcommand(
            App::new("remove")
                .about("Remove the specified schema from the configuration")
                .arg(Arg::new("INPUT").help("Schema to be removed").min_values(1)),
        )
        .subcommand(
            App::new("sync").about("Synchronize the configuration files with the installed schema"),
        )
        .subcommand(
            App::new("set-default")
                .about("Set the specified schema to be the default schema")
                .arg(
                    Arg::new("INPUT")
                        .help("Schema to be set as the default")
                        .required(true),
                ),
        )
        .subcommand(App::new("list").about("List installed schema"))
}
