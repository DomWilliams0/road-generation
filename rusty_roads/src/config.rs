use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use std::io;
use toml;

#[derive(Clone, Deserialize, Default)]
pub struct Config {
    pub width: u32,
    pub height: u32,

    pub growth_increment: Option<u32>,

    pub merge_range: f64,
}

impl Config {
    fn load_unsafe(path: &'static str) -> io::Result<Config> {
        let mut file = File::open(path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config: Config = match toml::from_str(&contents) {
            Ok(c) => c,
            Err(e) => {
                return Err(io::Error::new(io::ErrorKind::Other,
                                          String::from(format!("Parse error: {}",
                                                               e.description()))))
            }
        };

        Ok(config)
    }

    // TODO validate
    pub fn load(self, path: &'static str) -> (bool, Config) {
        match Config::load_unsafe(path) {
            Ok(config) => (true, config),
            Err(e) => {
                println!("Config error, keeping original settings ({:?})", e);
                (false, self)
            }
        }
    }
}
