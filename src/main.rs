use anyhow::{anyhow, Result};
use serde_yaml::{mapping::Mapping, Value};
use std::{fs, path::Path};

mod cli;

const CONFIG: &str = "/usr/share/rime-data/default.yaml";
const DATA_DIR: &str = "/usr/share/rime-data/";

fn main() -> Result<()> {
    let app = cli::build_cli().get_matches();
    let mut config = read_config()?;

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
        ("sync", _) => {
            let schema_list: Vec<_> = collect_installed_schemas(DATA_DIR)?
                .into_iter()
                .map(|s| {
                    let mut new_entry = Mapping::new();
                    new_entry.insert(Value::from("schema"), Value::from(s));
                    Value::from(new_entry)
                })
                .collect();
            let count = schema_list.len();
            config["schema_list"] = Value::Sequence(schema_list);
            write_config(&config)?;
            println!("Schema configuration updated. Found {} schemas.", count);
        }
        _ => {
            unreachable!()
        }
    }
    Ok(())
}

/// Read RIME schema configurations
fn read_config() -> Result<Value> {
    let config = fs::read(CONFIG)?;
    let config_data = serde_yaml::from_slice(&config)?;

    Ok(config_data)
}

/// Collect all schemas to a vector
fn list_schema(config: &Value) -> Result<Vec<&str>> {
    let schema_list = config
        .get("schema_list")
        .ok_or_else(|| anyhow!("No schema_list section found in the config file!"))?
        .as_sequence();
    if schema_list.is_none() {
        return Ok(Vec::new());
    }

    let mut schemas = Vec::new();
    for entry in schema_list.unwrap() {
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

/// Unwrap the schema value to a vector
fn schema_list_to_vec(mut config: Value) -> Result<(Vec<Value>, Value)> {
    let schema_list = config
        .get_mut("schema_list")
        .ok_or_else(|| anyhow!("No schema_list section found in the config file!"))?
        .as_sequence_mut()
        .map_or_else(|| Vec::new(), |x| x.to_owned());

    Ok((schema_list, config))
}

/// Collect all the installed schemas
fn collect_installed_schemas<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    let mut installed = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if let Some(schema) = entry
            .file_name()
            .to_string_lossy()
            .strip_suffix(".schema.yaml")
        {
            installed.push(schema.to_string());
        }
    }

    Ok(installed)
}

/// Write the schema list to disk
fn write_config(config: &Value) -> Result<()> {
    fs::write(CONFIG, serde_yaml::to_string(&config)?)?;
    Ok(())
}
