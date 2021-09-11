use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::{collections::HashMap, fs::File};
use std::{fs, path::Path};

mod cli;

const CONFIG: &str = "/usr/share/rime-data/default.yaml";
const DATA_DIR: &str = "/usr/share/rime-data/";

#[derive(Deserialize, Serialize, Debug)]
struct SchemaItem {
    schema: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct SchemaConfig {
    #[serde(default)]
    schema_list: Vec<SchemaItem>,
    #[serde(flatten)]
    other: HashMap<String, Value>,
}

fn main() -> Result<()> {
    let app = cli::build_cli().get_matches();
    let mut config = read_config()?;

    match app.subcommand() {
        ("add", Some(args)) => {
            let schema_list = &mut config.schema_list;
            for entry in args.values_of("INPUT").unwrap() {
                if schema_list.iter().any(|s| s.schema == entry) {
                    // exists
                    println!("Schema {:?} already exists in default.yaml", entry);
                    continue;
                }
                schema_list.push(SchemaItem {
                    schema: entry.to_owned(),
                });
            }
            // write config
            write_config(config)?;
        }
        ("list", _) => {
            for v in config.schema_list {
                println!("{}", v.schema);
            }
        }
        ("set-default", Some(args)) => {
            let schema_list = &mut config.schema_list;
            let entry = args.value_of("INPUT").unwrap();

            if let Some(index) = schema_list.iter().position(|v| v.schema == entry) {
                schema_list.swap(0, index);
                write_config(config)?;
            } else {
                println!("schema {:?} doesn’t not exist", entry);
            }
        }
        ("remove", Some(args)) => {
            let schema_list = &mut config.schema_list;
            for entry in args.values_of("INPUT").unwrap() {
                if let Some(index) = schema_list.iter().position(|v| v.schema == entry) {
                    schema_list.remove(index);
                } else {
                    println!("schema {:?} doesn’t not exist", entry);
                }
            }
            write_config(config)?;
        }
        ("sync", _) => {
            config.schema_list = collect_installed_schemas(DATA_DIR)?
                .into_iter()
                .map(|s| SchemaItem { schema: s })
                .collect();
            let count = config.schema_list.len();
            write_config(config)?;
            println!("Schema configuration updated. Found {} schemas.", count);
        }
        _ => {
            unreachable!()
        }
    }
    Ok(())
}

/// Read RIME schema configurations
fn read_config() -> Result<SchemaConfig> {
    let config = File::open(CONFIG)?;
    let config_data = serde_yaml::from_reader(&config)?;

    Ok(config_data)
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
fn write_config(config: SchemaConfig) -> Result<()> {
    fs::write(CONFIG, serde_yaml::to_vec(&config)?)?;
    Ok(())
}
