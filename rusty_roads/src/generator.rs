use super::{RoadError, RoadType, RoadmapSettings};
use kdtree::kdtree::*;

pub struct RoadMap {
    kdtree: Kdtree<Point>,
    // no graph
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    pos: [f64; 2],
}

#[derive(Debug)]
struct Road {
    from: Option<Point>,
    to: Option<Point>,
    road_type: RoadType,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { pos: [x, y] }
    }
}

impl KdtreePointTrait for Point {
    fn dims(&self) -> &[f64] {
        &self.pos
    }
}

impl Road {
    fn new(road_type: RoadType) -> Road {
        Road {
            road_type: road_type,
            from: None,
            to: None,
        }
    }

    fn from(mut self, point: Point) -> Self {
        self.from = Some(point);
        self
    }

    fn to(mut self, point: Point) -> Self {
        self.to = Some(point);
        self
    }
}


fn create_frontier() -> Vec<Road> {
    let mut vec: Vec<Road> = Vec::new();

    // TODO randomise
    let a = Point::new(0., 0.);
    let b = Point::new(10., 10.);
    let road = Road::new(RoadType::Large).from(a).to(b);
    vec.push(road);

    vec
}

impl RoadMap {
    pub fn generate(settings: &RoadmapSettings) -> Result<RoadMap, RoadError> {

        let mut frontier = create_frontier();
        let mut frontier_points = frontier
            .iter()
            .fold(Vec::with_capacity(frontier.len() * 2),
                  |mut acc, ref road| {
                      road.from.map_or((), |p| acc.push(p));
                      road.to.map_or((), |p| acc.push(p));
                      acc
                  });

        let roadmap = RoadMap { kdtree: Kdtree::new(&mut frontier_points) };

        while let Some(_point) = frontier.pop() {}


        Ok(roadmap)
    }
}
