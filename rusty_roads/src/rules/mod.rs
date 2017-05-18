use {Road, RoadType, Point};
use cgmath::{Vector2, Point2};
use cgmath::prelude::*;

mod grid;
mod organic;

enum GenerationRule {
    Grid,
    // Radial,
    Organic,
}

type Proposal = fn(&Point2<f64>, &Vector2<f64>, RoadType, bool, &mut Vec<Road>);

pub fn propose_roads(road: &Road, branch: bool, out: &mut Vec<Road>) {

    let from = Point2::from(road.from().unwrap().pos);
    let to = Point2::from(road.to().unwrap().pos);
    let vec = (to - from).normalize();

    let rule = get_rule(road.to().as_ref().unwrap());
    if let Some(generator) = get_generator(&rule) {
        (generator)(&to, &vec, road.road_type(), branch, out);
    }
}

fn get_rule(point: &Point) -> GenerationRule {
    if point.x() < 400.0 || point.x() > 600.0 {
        // arbitrary nonsense
        GenerationRule::Grid
    } else {
        GenerationRule::Organic
    }
}

fn get_generator(rule: &GenerationRule) -> Option<Proposal> {
    match *rule {
        GenerationRule::Grid => Some(grid::propose_branching_roads),
        GenerationRule::Organic => Some(organic::propose_branching_roads),
        // _ => None,
    }
}
