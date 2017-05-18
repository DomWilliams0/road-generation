extern crate kdtree;
extern crate rand;
extern crate cgmath;

pub mod generator;
mod rules;

use kdtree::kdtree::Kdtree;

#[derive(Debug)]
pub enum RoadError {
    Args(String),
    Settings(&'static str),
    Unknown(&'static str),
}


pub struct RoadMap {
    kdtree: Kdtree<Point>,
    roads: Vec<Road>,
    frontier: Vec<Road>,

    settings: RoadmapSettings,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub pos: [f64; 2],
}

#[derive(Debug)]
pub struct Road {
    from: Option<Point>,
    to: Option<Point>,
    road_type: RoadType,
    fuel: u32,
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

    pub fn size(&mut self, w: i32, h: i32) -> &mut RoadmapBuilder {
        self.settings.width = w;
        self.settings.height = h;
        self
    }

    pub fn increment(&mut self, increment: Option<i32>) -> &mut RoadmapBuilder {
        self.settings.increment = increment;
        self
    }


    pub fn create(&self) -> Result<RoadMap, RoadError> {
        RoadMap::create(self.settings.clone())
    }
}
