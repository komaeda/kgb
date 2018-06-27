use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub name: String,
    pub destination: Option<PathBuf>,
}

pub fn read_config(source: &str) -> io::Result<Config> {
    let file_name = OsStr::new("_config.toml");
    let path: PathBuf = Path::new(source).join(file_name);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config = toml::from_str(contents.as_str()).unwrap();
    Ok(config)
}
