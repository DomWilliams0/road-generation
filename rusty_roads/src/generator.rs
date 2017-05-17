use super::{RoadError, RoadType, RoadmapSettings};
use kdtree::kdtree::*;
use rules;
use rand::{thread_rng, Rng};

pub struct RoadMap {
    kdtree: Kdtree<Point>,
    roads: Vec<Road>,
    frontier: Vec<Road>,

    settings: RoadmapSettings
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

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { pos: [x, y] }
    }

    pub fn out_of_range() -> Point {
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
    pub fn new_with_points(road_type: RoadType, from: Point, to: Point) -> Road {
        Road {
            road_type: road_type,
            from: Some(from),
            to: Some(to),
            fuel: 1,
        }
    }
    pub fn new(road_type: RoadType) -> Road {
        Road {
            road_type: road_type,
            from: None,
            to: None,
            fuel: 1,
        }
    }

    fn _set_from(&mut self, point: Point) {
        self.from = Some(point);
    }

    fn set_to(&mut self, point: Point) {
        self.to = Some(point);
    }

    pub fn points(&self) -> (Option<Point>, Option<Point>) {
        (self.from, self.to)
    }

    pub fn to(&self) -> &Option<Point> {
        &self.to
    }

    pub fn from(&self) -> &Option<Point> {
        &self.from
    }

    pub fn road_type(&self) -> RoadType {
        self.road_type
    }

    pub fn set_fuel(&mut self, fuel: u32) {
        self.fuel = fuel;
    }

    pub fn take_fuel(&mut self) -> bool {
        if self.fuel > 0 {
            self.fuel -= 1;
        }
        self.fuel <= 0
    }
}


fn create_frontier() -> Vec<Road> {
    let mut vec: Vec<Road> = Vec::new();

    // TODO randomise
    let a = Point::new(100., 100.);
    let b = Point::new(100., 120.);
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
    pub fn generate(settings: RoadmapSettings) -> Result<RoadMap, RoadError> {
        validate_settings(&settings)?;

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
        self.settings.width
    }

    pub fn height(&self) -> i32 {
        self.settings.height
    }

    fn new(settings: RoadmapSettings, frontier: Vec<Road>) -> RoadMap {
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
            settings: settings,
            roads: Vec::new(),
            kdtree: Kdtree::new(&mut frontier_points),
        }
    }

    fn generate_roads(&mut self) -> Result<(), RoadError> {
        while let Some(mut road) = self.frontier.pop() {

            let (accepted, did_merge) = self.accept_local_constraints(&mut road);
            if !accepted {
                // println!("Rejecting {:?}", road);
                continue;
            }

            // tweaked out of range
            if !self.is_in_range(&road) {
                continue;
            }

            // propose some more
            if !did_merge {
                let branch = road.take_fuel();
                let mut proposed = self.propose_with_global_goals(&road, branch);
                self.frontier.append(&mut proposed);
            }

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

        let w = (self.width() + 1) as f64;
        let h = (self.height() + 1) as f64;

        from.pos[0] >= 0. && from.pos[0] < w && from.pos[1] >= 0. && from.pos[1] < h &&
        to.pos[0] >= 0. && to.pos[0] < w && to.pos[1] >= 0. && to.pos[1] < h
    }

    // returns (accepted, merged)
    fn accept_local_constraints(&self, road: &mut Road) -> (bool, bool) {
        const MERGE_RANGE: f64 = 18.;

        // out of range
        if !self.is_in_range(road) {
            return (false, false);
        }

        // merge with nearby
        let mut merged = false;
        let merger = road.to.unwrap();
        if self.kdtree.has_neighbor_in_range(&merger, MERGE_RANGE) {
            let nearest = self.kdtree.nearest_search(&merger);

            // self, therefore this is a duplicate
            if nearest == merger {
                return (false, false);
            }

            // other end of self
            if nearest == road.from.unwrap() {
                return (false, false);
            }

            // println!("Merging");

            // merge with the new closest
            road.set_to(nearest);
            merged = true;
        }

        (true, merged)
    }

    fn propose_with_global_goals(&self, road: &Road, branch: bool) -> Vec<Road> {

        let mut vec: Vec<Road> = Vec::new();

        rules::propose_roads(road, branch, &mut vec);

        if branch {
            let mut rng = thread_rng();
            for r in vec.iter_mut() {
                let fuel = rng.gen_range(1, 4);
                r.set_fuel(fuel);
            }
        }

        vec
    }
}
