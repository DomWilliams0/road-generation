use super::{RoadError, RoadType, RoadmapSettings};
use kdtree::kdtree::*;

pub struct RoadMap {
    kdtree: Kdtree<Point>,
    roads: Vec<Road>,
    frontier: Vec<Road>,

    // TODO put settings object directly in here instead
    width: i32,
    height: i32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pos: [f64; 2],
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

    fn out_of_range() -> Point {
        Point { pos: [-1.; 2] }
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.pos[0]
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.pos[1]
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
    fn new_with_points(road_type: RoadType, from: Point, to: Point) -> Road {
        Road {
            road_type: road_type,
            from: Some(from),
            to: Some(to),
        }
    }
    fn new(road_type: RoadType) -> Road {
        Road {
            road_type: road_type,
            from: None,
            to: None,
        }
    }

    fn from(&mut self, point: Point) {
        self.from = Some(point);
    }

    fn to(&mut self, point: Point) {
        self.to = Some(point);
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
    let road = Road::new_with_points(RoadType::Large, a, b);
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
        let mut roadmap = RoadMap::new(settings, frontier);
        match roadmap.generate_roads() {
            Ok(_) => Ok(roadmap),
            Err(e) => Err(e),
        }

    }

    pub fn roads(&self) -> &Vec<Road> {
        &self.roads
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    fn new(settings: &RoadmapSettings, frontier: Vec<Road>) -> RoadMap {
        let mut frontier_points = frontier
            .iter()
            .fold(Vec::with_capacity(frontier.len() * 2),
                  |mut acc, ref road| {
                      road.from.map_or((), |p| acc.push(p));
                      road.to.map_or((), |p| acc.push(p));
                      acc
                  });

        frontier_points.pop();

        RoadMap {
            frontier: frontier,
            width: settings.width,
            height: settings.height,
            roads: Vec::new(),
            kdtree: Kdtree::new(&mut frontier_points),
        }
    }

    fn generate_roads(&mut self) -> Result<(), RoadError> {
        while let Some(mut road) = self.frontier.pop() {

            if !self.accept_local_constraints(&mut road) {
                continue;
            }

            // tweaked out of range
            if !self.is_in_range(&road) {
                continue;
            }

            // propose some more
            let mut proposed = self.propose_with_global_goals(&road);
            self.frontier.append(&mut proposed);

            // add self to world
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

    // TODO check only from?
    fn is_in_range(&self, road: &Road) -> bool {
        let from = road.from.unwrap_or(Point::out_of_range());
        let to = road.to.unwrap_or(Point::out_of_range());

        let w = self.width as f64;
        let h = self.height as f64;

        from.pos[0] >= 0. && from.pos[0] < w && from.pos[1] >= 0. && from.pos[1] < h &&
        to.pos[0] >= 0. && to.pos[0] < w && to.pos[1] >= 0. && to.pos[1] < h
    }

    fn accept_local_constraints(&self, road: &mut Road) -> bool {
        // out of range
        if !self.is_in_range(road) {
            return false;
        }

        // TODO check if from already exists?

        // merge with nearby
        let MERGE_RANGE = 10f64;
        let merger = road.to.unwrap();
        if self.kdtree.has_neighbor_in_range(&merger, MERGE_RANGE) {
            let nearest = self.kdtree.nearest_search(&merger);

            // self, therefore this is a duplicate
            if nearest == merger {
                return false;
            }

            // other end of self
            if nearest == road.from.unwrap() {
                return false;
            }

            // merge with the new closest
            road.to(nearest);
        }

        true
    }

    fn propose_with_global_goals(&self, road: &Road) -> Vec<Road> {

        let mut vec: Vec<Road> = Vec::new();

        // simple and stupid
        vec.push(Road::new_with_points(road.road_type.clone(),
                                       road.to.unwrap(),
                                       Point::new(road.to.unwrap().x() + 10.,
                                                  road.to.unwrap().y() + 5.)));

        vec
    }
}
