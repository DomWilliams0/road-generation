extern crate rusty_roads;

use rusty_roads::{RoadError, RoadMap};
use std::process;

fn main() {

    match run() {
        Err(err) => {
            println!("Error: {:?}", err);
            process::exit(1);
        }

        Ok(_) => process::exit(0),
    }

}

fn run() -> Result<(), RoadError> {

    let roadmap = rusty_roads::RoadmapBuilder::new()
        .size(600, 960)
        .generate()?;

    render(&roadmap)
}

fn render(roadmap: &RoadMap) -> Result<(), RoadError> {

    let roads = roadmap.roads();
    println!("TODO: render {} roads", roads.len());

    Ok(())
}
