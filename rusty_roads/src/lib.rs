extern crate kdtree;
extern crate rand;
extern crate cgmath;

pub mod generator;
pub use generator::RoadMap;
mod rules;

#[derive(Debug)]
pub enum RoadError {
    Args(String),
    Settings(&'static str),
    Unknown(&'static str),
}


pub struct RoadmapBuilder {
    pub settings: RoadmapSettings,
}

#[derive(Clone)]
pub struct RoadmapSettings {
    width: i32,
    height: i32,
    increment: Option<i32>,
}

#[derive(Debug, Copy, Clone)]
pub enum RoadType {
    Small,
    Medium,
    Large,
}

impl RoadmapBuilder {
    pub fn new() -> RoadmapBuilder {
        RoadmapBuilder {
            settings: RoadmapSettings {
                width: 256,
                height: 256,
                increment: None,
            },
        }
    }

    pub fn size<'a>(&'a mut self, w: i32, h: i32) -> &'a mut RoadmapBuilder {
        self.settings.width = w;
        self.settings.height = h;
        self
    }

    pub fn increment<'a>(&'a mut self, increment: Option<i32>) -> &'a mut RoadmapBuilder {
        self.settings.increment = increment;
        self
    }


    pub fn create(&self) -> Result<RoadMap, RoadError> {
        RoadMap::create(self.settings.clone())
    }
}
