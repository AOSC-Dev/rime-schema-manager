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

    let config = read_config()?;

    match app.subcommand() {
        ("add", Some(args)) => {
            let (schema_list, mut config) = schema_list_to_vec(config).unwrap();
            for entry in args.values_of("INPUT").unwrap() {
                if list_schema(&config)?.contains(&entry) {
                    // exists
                    // continue;
                    println!("Schema {:?} already exist in default.yaml", entry);
                    continue;
                }
                let mut new_entry = Mapping::new();
                new_entry.insert(Value::from("schema"), Value::from(entry));
                schema_list.push(Value::from(new_entry));
            }
            // write config
            *config.get_mut("schema_list").unwrap() = Value::Sequence(schema_list.to_vec());
            write_config(&config)?;
        }
        ("list", _) => {
            let schema_list = list_schema(&config)?;
            for v in &schema_list {
                println!("{}", v);
            }
        }
        ("set-default", Some(args)) => {
            let (schema_list, mut config) = schema_list_to_vec(config).unwrap();
            let entry = args.value_of("INPUT").unwrap();

            if let Some(index) = schema_list
                .iter()
                .position(|v| v.as_str().unwrap_or("") == entry)
            {
                schema_list.remove(index);
                let mut default_entry = Mapping::new();
                default_entry.insert(Value::from("schema"), Value::from(entry));
                let mut new_schema_list = vec![Value::from(default_entry)];
                new_schema_list.extend_from_slice(schema_list);
                *config.get_mut("schema_list").unwrap() = Value::Sequence(new_schema_list.to_vec());
                write_config(&config)?;
            } else {
                println!("schema {:?} doesn’t not exist", entry);
            }
        }
        ("remove", Some(args)) => {
            let (schema_list, mut config) = schema_list_to_vec(config).unwrap();
            for entry in args.values_of("INPUT").unwrap() {
                if let Some(index) = schema_list
                    .iter()
                    .position(|v| v.as_str().unwrap_or("") == entry)
                {
                    schema_list.remove(index);
                } else {
                    println!("Schema {:?} doesn’t not exist in default.yaml", entry);
                }
            }
            *config.get_mut("schema_list").unwrap() = Value::Sequence(schema_list.to_vec());
            write_config(&config)?;
        }
        _ => {
            unreachable!()
        }
    }

    Ok(())
}

fn read_config() -> Result<Value> {
    let config = fs::read(CONFIG)?;
    let config_data = serde_yaml::from_slice(&config)?;

    Ok(config_data)
}

fn list_schema(config: &Value) -> Result<Vec<&str>> {
    let mut schemas: Vec<&str> = Vec::new();
    for entry in config
        .get("schema_list")
        .ok_or_else(|| anyhow!("No schema_list section found in the config file!"))?
        .as_sequence()
        .ok_or_else(|| anyhow!("schema_list is not an array!"))?
    {
        if let Some(schema) = entry.get("schema") {
            schemas.push(
                schema
                    .as_str()
                    .ok_or_else(|| anyhow!("schema name is not a string"))?,
            );
        }
    }

    Ok(schemas)
}

fn schema_list_to_vec<'a>(mut config: Value) -> Result<(&'a mut Vec<Value>, Value)> {
    let schema_list = config
        .get_mut("schema_list")
        .ok_or_else(|| anyhow!("No schema_list section found in the config file!"))?;
    let schema_list = schema_list.as_sequence_mut().unwrap();

    Ok((schema_list, config))
}

fn write_config(config: &Value) -> Result<()> {
    fs::write(CONFIG, serde_yaml::to_string(&config)?)?;

    Ok(())
}
