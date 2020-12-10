extern crate serde_yaml;

use anyhow::{ format_err, Result };
use std::fs;
use clap::{ Arg, App };
use serde_yaml::Value as yamlValue;


fn main() {
    let app = App::new("Rime Schema Manager")
        .version("0.1")
        .author("AOSC-Dev")
        .about("Rime Schema Add/Remove/Echo")
        .subcommand(App::new("add")
            .about("Add Schema to default.yaml")
            .arg(Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .empty_values(false)
                .multiple(true)
                .index(1)))
        .subcommand(App::new("echo")
            .about("aaa"))
        .get_matches();

    if let Some(add) = app.subcommand_matches("add") {
        let result: Vec<_> = add.values_of("INPUT").unwrap().collect();
        println!("{:?}", &result);
    } else if let Some(_echo) = app.subcommand_matches("echo") {
        println!("{:?}", read_config());
    }
}

fn read_config() -> Result<std::vec::Vec<serde_yaml::Value>> {
    let config_string = fs::read_to_string("/usr/share/rime-data/default.yaml")
        .expect("Something went wrong reading the file");

    let config_data: yamlValue = serde_yaml::from_str(&config_string).expect("This file not yaml!");
    let schema_list = match &config_data["schema_list"] {
        yamlValue::Sequence(m) => m.to_vec(),
        _ => {
            return Err(format_err!("can not see schema_list!"));
        }
    };

    for v in &schema_list {
        println!("{:?}", v["schema"]);
    }

    Ok(schema_list)
}

