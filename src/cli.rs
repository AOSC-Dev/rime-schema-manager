use clap::{crate_version, App, AppSettings, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new("Rime Schema Manager")
        .version(crate_version!())
        .author("AOSC-Dev")
        .about("Rime Schema Manager")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("add")
                .about("Add the specified schema to the configuration")
                .arg(
                    Arg::with_name("INPUT")
                        .help("Sets the input file to use")
                        .min_values(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .about("Remove the specified schema from the configuration")
                .arg(
                    Arg::with_name("INPUT")
                        .help("Schema to be removed")
                        .min_values(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("set-default")
                .about("Set the specified schema to be the default schema")
                .arg(
                    Arg::with_name("INPUT")
                        .help("Schema to be set as the default")
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("list").about("List installed schema"))
}
