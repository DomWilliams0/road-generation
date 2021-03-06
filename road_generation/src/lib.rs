extern crate kdtree;
extern crate rand;
extern crate cgmath;
extern crate smallvec;

#[macro_use]
extern crate serde_derive;
extern crate toml;

pub mod generator;
pub mod config;
mod rules;

use kdtree::kdtree::Kdtree;
use std::collections::VecDeque;
pub use config::Config;

#[derive(Debug)]
pub enum RoadError {
    Args(String),
    Settings(String),
    Unknown(&'static str),
}


pub struct RoadMap {
    kdtree: Kdtree<Point>,
    roads: Vec<Road>,
    frontier: VecDeque<Road>,

    config: config::Config,
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

#[derive(Debug, Copy, Clone)]
pub enum RoadType {
    Small = 0,
    Medium,
    Large,
}
