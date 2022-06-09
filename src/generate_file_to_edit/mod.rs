#![allow(dead_code)]

use tempfile::tempfile;
use serde::{Serialize, Deserialize};
use serde_yaml;
use std::io::Write;

/// Generates a yamlfile with template values to fill in and opens $EDITOR to edit them
pub fn generate_and_inspect_yamlfile() -> Result<(), std::io::Error>{
    let serialize_this = SerializeMe::new("Willis", "20221214");
    let tempfile = tempfile::tempfile()?;
    // serde_yaml::to_writer(tempfile, &serialize_this)?;
    preview_yaml_file();

    Ok(())
}

fn dump_yaml_to_tempfile() {
}

fn preview_yaml_file() {
}

/// A type to serialize for editing
#[derive(Debug, Serialize, Deserialize)]
struct SerializeMe {
    name: Option<String>,
    date_progression: Option<String>
}

impl SerializeMe {
    fn new(name: &str, date_progression: &str) -> Self {
        Self {
            name: Some(String::from(name)),
            date_progression: Some(String::from(date_progression))
        }
    }
}

impl Default for SerializeMe {
    fn default() -> Self {
        SerializeMe {
            name: None,
            date_progression: None
        }
    }
}
