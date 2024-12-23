use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::{collections::HashMap, fs::File};
use std::{fs, path::Path};

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

/// Rime Schema Manager
#[derive(Parser, Debug)]
#[command(version, about, author = "AOSC-Dev", long_about = None)]
enum RimeSchemaManagerCli {
    /// Add the specified schema to the configuration
    #[command()]
    Add {
        /// Sets the input file to use
        #[arg(required = true)]
        input: Vec<String>,
    },
    /// Remove the specified schema from the configuration
    #[command()]
    Remove {
        /// Schema to be removed
        #[arg(required = true)]
        input: Vec<String>,
    },
    /// Synchronize the configuration files with the installed schema
    #[command()]
    Sync,
    /// Set the specified schema to be the default schema
    #[command()]
    SetDefault {
        /// Schema to be set as the default
        #[arg(required = true)]
        input: String,
    },
    /// List installed schema
    #[command()]
    List,
}

fn main() -> Result<()> {
    let app = RimeSchemaManagerCli::parse();
    let mut config = read_config()?;

    match app {
        RimeSchemaManagerCli::Add { input } => {
            let schema_list = &mut config.schema_list;
            for entry in input {
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
        RimeSchemaManagerCli::List => {
            for v in config.schema_list {
                println!("{}", v.schema);
            }
        }
        RimeSchemaManagerCli::SetDefault { input } => {
            let schema_list = &mut config.schema_list;
            if let Some(index) = schema_list.iter().position(|v| v.schema == input) {
                schema_list.swap(0, index);
                write_config(config)?;
            } else {
                println!("schema {:?} does not exist", input);
            }
        }
        RimeSchemaManagerCli::Remove { input } => {
            let schema_list = &mut config.schema_list;
            for entry in input {
                if let Some(index) = schema_list.iter().position(|v| v.schema == entry) {
                    schema_list.remove(index);
                } else {
                    println!("schema {:?} does not exist", entry);
                }
            }
            write_config(config)?;
        }
        RimeSchemaManagerCli::Sync => {
            config.schema_list = collect_installed_schemas(DATA_DIR)?
                .into_iter()
                .map(|s| SchemaItem { schema: s })
                .collect();
            let count = config.schema_list.len();
            write_config(config)?;
            println!("Schema configuration updated. Found {} schemas.", count);
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
    let file = fs::File::create(CONFIG)?;
    serde_yaml::to_writer(file, &config)?;

    Ok(())
}
