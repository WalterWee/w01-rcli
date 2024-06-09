use std::fs;

use anyhow::{Ok, Result};
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::opts::OutputFormat;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: String, output_format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let head = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        let json_value: Value = head.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }
    match output_format {
        OutputFormat::Json => {
            let json: String = serde_json::to_string_pretty(&ret)?;
            fs::write(output, json)?;
        }
        OutputFormat::Yaml => {
            let yaml: String = serde_yaml::to_string(&ret)?;
            fs::write(output, yaml)?;
        }
    }
    Ok(())
}
