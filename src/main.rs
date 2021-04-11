use anyhow::{anyhow, Result};
use serde_yaml::{mapping::Mapping, Value};
use std::fs;

mod cli;

const CONFIG: &str = "/usr/share/rime-data/default.yaml";

fn main() -> Result<()> {
    let app = cli::build_cli().get_matches();
    let config = read_config()?;

    match app.subcommand() {
        ("add", Some(args)) => {
            let (mut schema_list, mut config) = schema_list_to_vec(config).unwrap();
            for entry in args.values_of("INPUT").unwrap() {
                if list_schema(&config)?.contains(&entry) {
                    // exists
                    println!("Schema {:?} already exists in default.yaml", entry);
                    continue;
                }
                let mut new_entry = Mapping::new();
                new_entry.insert(Value::from("schema"), Value::from(entry));
                schema_list.push(Value::from(new_entry));
            }
            // write config
            config["schema_list"] = Value::Sequence(schema_list);
            write_config(&config)?;
        }
        ("list", _) => {
            let schema_list = list_schema(&config)?;
            for v in &schema_list {
                println!("{}", v);
            }
        }
        ("set-default", Some(args)) => {
            let (mut schema_list, mut config) = schema_list_to_vec(config).unwrap();
            let entry = args.value_of("INPUT").unwrap();

            if let Some(index) = schema_list
                .iter()
                .position(|v| v["schema"].as_str().unwrap_or("") == entry)
            {
                schema_list.swap(0, index);
                config["schema_list"] = Value::Sequence(schema_list);
                write_config(&config)?;
            } else {
                println!("schema {:?} doesn’t not exist", entry);
            }
        }
        ("remove", Some(args)) => {
            let (mut schema_list, mut config) = schema_list_to_vec(config).unwrap();
            for entry in args.values_of("INPUT").unwrap() {
                if let Some(index) = schema_list
                    .iter()
                    .position(|v| v["schema"].as_str().unwrap_or("") == entry)
                {
                    schema_list.remove(index);
                } else {
                    println!("schema {:?} doesn’t not exist", entry);
                }
            }
            config["schema_list"] = Value::Sequence(schema_list.to_vec());
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

fn schema_list_to_vec(mut config: Value) -> Result<(Vec<Value>, Value)> {
    let schema_list = config
        .get_mut("schema_list")
        .ok_or_else(|| anyhow!("No schema_list section found in the config file!"))?
        .as_sequence_mut()
        .unwrap()
        .to_owned();

    Ok((schema_list, config))
}

fn write_config(config: &Value) -> Result<()> {
    fs::write(CONFIG, serde_yaml::to_string(&config)?)?;
    Ok(())
}
