use super::{RoadError, RoadType, RoadmapSettings};
use kdtree::kdtree::*;

pub struct RoadMap {
    kdtree: Kdtree<Point>,
    roads: Vec<Road>,
    frontier: Vec<Road>,
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
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { pos: [x, y] }
    }
}

impl KdtreePointTrait for Point {
    #[inline]
    fn dims(&self) -> &[f64] {
        &self.pos
    }
}

impl From<[f64; 2]> for Point {
    fn from(dims: [f64; 2]) -> Point {
        Point { pos: dims }
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

    pub fn points(&self) -> (Option<Point>, Option<Point>) {
        (self.from, self.to)
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
fn validate_settings(settings: &RoadmapSettings) -> Result<(), RoadError> {
    if settings.width < 50 {
        return Err(RoadError::Settings("Width must be at least 50"));
    }
    if settings.height < 50 {
        return Err(RoadError::Settings("Height must be at least 50"));
    }

    Ok(())
}


impl RoadMap {
    pub fn generate(settings: &RoadmapSettings) -> Result<RoadMap, RoadError> {
        validate_settings(settings)?;

        let frontier = create_frontier();
        let mut roadmap = RoadMap::new(frontier);
        match roadmap.generate_roads(settings) {
            Ok(_) => Ok(roadmap),
            Err(e) => Err(e),
        }

    }

    pub fn roads(&self) -> &Vec<Road> {
        &self.roads
    }

    fn new(frontier: Vec<Road>) -> RoadMap {
        let mut frontier_points = frontier
            .iter()
            .fold(Vec::with_capacity(frontier.len() * 2),
                  |mut acc, ref road| {
                      road.from.map_or((), |p| acc.push(p));
                      road.to.map_or((), |p| acc.push(p));
                      acc
                  });

        RoadMap {
            frontier: frontier,
            roads: Vec::new(),
            kdtree: Kdtree::new(&mut frontier_points),
        }
    }

    fn generate_roads(&mut self, settings: &RoadmapSettings) -> Result<(), RoadError> {
        while let Some(mut road) = self.frontier.pop() {

            if !self.accept_local_constraints(&mut road) {
                continue;
            }

            // tweaked out of range
            if !self.is_in_range(&road) {
                continue;
            }

            // add to world
            self.add_road(road);
        }



        Ok(())
    }

    fn add_road(&mut self, road: Road) {

        let a = road.from.unwrap().pos;
        let b = road.to.unwrap().pos;

        self.kdtree.insert_node(Point::from(a));
        self.kdtree.insert_node(Point::from(b));

        self.roads.push(road);
    }

    fn is_in_range(&self, road: &Road) -> bool {
        true
    }

    fn accept_local_constraints(&self, road: &mut Road) -> bool {
        true
    }
}
