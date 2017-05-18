use RoadType;
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use std::io;
use toml;

#[derive(Clone, Deserialize, Default)]
pub struct Config {
    pub window: WindowConfig,
    generation: GenerationConfigs,
}

#[derive(Clone, Deserialize, Default)]
pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
    pub growth_increment: Option<u32>,
}

#[derive(Clone, Deserialize, Default)]
struct GenerationConfigs {
    large: GenerationConfig,
    medium: Option<GenerationConfig>,
    small: Option<GenerationConfig>,
}


#[derive(Clone, Deserialize, Default)]
pub struct GenerationConfig {
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

    // defaults to Large config if not specified in config
    pub fn generation(&self, road_type: &RoadType) -> &GenerationConfig {
        let gen = match *road_type {
            RoadType::Large => Some(&self.generation.large),
            RoadType::Medium => self.generation.medium.as_ref(),
            RoadType::Small => self.generation.small.as_ref(),
        };

        gen.unwrap_or(&self.generation.large)
    }
}
