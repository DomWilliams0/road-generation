extern crate rusty_roads;

fn main() {

    let r = rusty_roads::RoadmapBuilder::new().size(600, 960).generate();

    println!("Generated {:?}", r);
}
