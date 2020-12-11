extern crate serde_yaml;

use anyhow::{Result, format_err};
use std::{collections::HashMap, fs};
use clap::{Arg, App};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Debug)]
struct Config {
    schema_list: Vec<HashMap<String, String>>,
}

#[derive(Clone, Deserialize, Debug)]
struct Schema {
    schema: String,
}

const CONFIG: &str = "/usr/share/rime-data/default.yaml";
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
        let schema_list = read_config().unwrap();
        for v in &schema_list {
            println!("{:?}", v.get("schema").unwrap());
        }
    }
}

fn read_config() -> Result<Vec<HashMap<String, String>>> {
    let config = fs::read_to_string(CONFIG)?;
    let config_data: Config = serde_yaml::from_str(&config)?;
    let schema_list = config_data.schema_list;
    Ok(schema_list)
}