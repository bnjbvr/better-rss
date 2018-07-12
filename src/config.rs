extern crate serde_yaml;

use std::fs::File;
use std::io::prelude::*;

use utils::GenResult;

#[derive(Debug, PartialEq, Deserialize)]
pub struct ConfigEntry {
    pub feed_name: String,
    pub num_entries: u32
}

type Configuration = Vec<ConfigEntry>;

pub fn read_config() -> GenResult<Configuration> {
    let mut file = File::open("config.yaml")?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    let config = serde_yaml::from_str(file_content.as_str())?;
    Ok(config)
}
