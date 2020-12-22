use anyhow::{anyhow, Result};
use clap::{App, Arg};
use serde_yaml::{mapping::Mapping, Value};
use std::{collections::HashSet, fs};

const CONFIG: &str = "/usr/share/rime-data/default.yaml";

fn main() -> Result<()> {
    let app = App::new("Rime Schema Manager")
        .version("0.1")
        .author("AOSC-Dev")
        .about("Rime Schema Manager")
        .subcommand(
            App::new("add")
                .about("Add the specified schema to the configuration")
                .arg(
                    Arg::with_name("INPUT")
                        .help("Sets the input file to use")
                        .min_values(1),
                ),
        )
        .subcommand(
            App::new("remove")
                .about("Remove the specified schema from the configuration")
                .arg(
                    Arg::with_name("INPUT")
                        .help("Schema to be removed")
                        .min_values(1),
                ),
        )
        .subcommand(
            App::new("set-default")
                .about("Set the specified schema to be the default schema")
                .arg(
                    Arg::with_name("INPUT")
                        .help("Schema to be set as the default")
                        .required(true),
                ),
        )
        .subcommand(App::new("list").about("List installed schema"))
        .get_matches();

    let mut config = read_config()?;

    match app.subcommand() {
        ("add", Some(args)) => {
            let mut config_clone = config.clone();
            let schema_lookup = list_schema(&config_clone)?;
            let schema_list = config
                .get_mut("schema_list")
                .ok_or_else(|| anyhow!("No schema_list section found in the config file!"))?;
            let schema_list = schema_list.as_sequence_mut().unwrap();
            for entry in args.values_of("INPUT").unwrap() {
                if schema_lookup.contains(&entry) {
                    // exists
                    // continue;
                    println!("Schema: {:?} already exist in default.yaml", entry);
                } else {
                    let mut new_entry = Mapping::new();
                    new_entry.insert(Value::from("schema"), Value::from(entry));
                    schema_list.push(Value::from(new_entry));
                }
            }
            // write config
            *config_clone
                .get_mut("schema_list")
                .unwrap() = Value::Sequence(schema_list.to_vec());
            write_config(&config_clone)?;
        }
        ("list", _) => {
            let schema_list = list_schema(&config)?;
            for v in &schema_list {
                println!("{}", v);
            }
        }
        ("set-default", Some(args)) => {
            // TODO
        }
        ("remove", Some(args)) => {
            // TODO
        }
        _ => {}
    }

    Ok(())
}

fn read_config() -> Result<Value> {
    let config = fs::read(CONFIG)?;
    let config_data = serde_yaml::from_slice(&config)?;

    Ok(config_data)
}

fn list_schema(config: &Value) -> Result<HashSet<&str>> {
    let mut schemas: HashSet<&str> = HashSet::new();
    for entry in config
        .get("schema_list")
        .ok_or_else(|| anyhow!("No schema_list section found in the config file!"))?
        .as_sequence()
        .ok_or_else(|| anyhow!("schema_list is not an array!"))?
    {
        if let Some(schema) = entry.get("schema") {
            schemas.insert(
                schema
                    .as_str()
                    .ok_or_else(|| anyhow!("schema name is not a string"))?,
            );
        }
    }

    Ok(schemas)
}

fn write_config(config: &Value) -> Result<()> {
    fs::write(CONFIG, serde_yaml::to_string(&config)?)?;

    Ok(())
}
