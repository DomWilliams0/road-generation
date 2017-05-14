extern crate rusty_roads;

fn main() {

    let _ = rusty_roads::RoadmapBuilder::new().size(600, 960).generate();
}
