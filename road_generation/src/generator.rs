use kdtree::kdtree::*;
use {Point, Road, RoadType, RoadMap, RoadError};
use config::Config;
use rules;
use rand::{thread_rng, Rng};
use std::collections::VecDeque;

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
        self.fuel == 0
    }

    pub fn fuel(&self) -> u32 {
        self.fuel
    }
}


fn create_frontier(config: &Config) -> Vec<Road> {
    let mut vec: Vec<Road> = Vec::new();
    let mut rng = thread_rng();

    let road_type = RoadType::Large;
    let a = Point::new(rng.gen_range(0.0, config.window.width as f64),
                       rng.gen_range(0.0, config.window.height as f64));
    let b = Point::new(a.x() + config.generation(&road_type).road_length, a.y());

    let road = Road::new_with_points(RoadType::Large, a, b);
    vec.push(road);

    vec
}

impl RoadMap {
    pub fn new(config: Config) -> Result<RoadMap, RoadError> {
        let frontier = create_frontier(&config);
        Ok(RoadMap::with_frontier(config, frontier))

    }

    pub fn roads(&self) -> &Vec<Road> {
        &self.roads
    }

    pub fn width(&self) -> u32 {
        self.config.window.width
    }

    pub fn height(&self) -> u32 {
        self.config.window.height
    }

    fn with_frontier(config: Config, frontier: Vec<Road>) -> RoadMap {
        let mut frontier_points = frontier
            .iter()
            .fold(Vec::with_capacity(frontier.len() * 2), |mut acc, road| {
                road.from.map_or((), |p| acc.push(p));
                road.to.map_or((), |p| acc.push(p));
                acc
            });

        frontier_points.pop();

        RoadMap {
            frontier: VecDeque::from(frontier),
            config: config,
            roads: Vec::new(),
            kdtree: Kdtree::new(&mut frontier_points),
        }
    }

    pub fn advance(&mut self) -> Result<(), RoadError> {
        // TODO use Option properly dammit
        let counting = self.config.window.growth_increment.is_some();
        let mut count = self.config.window.growth_increment.unwrap_or(1);

        while count > 0 {
            if counting {
                count -= 1;
            }

            let popped = self.frontier.pop_front();
            if popped.is_none() {
                break;
            }

            let mut road = popped.unwrap();

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
                for r in proposed.drain(..) {
                  self.frontier.push_back(r);
                }
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
        let from = road.from.unwrap_or_else(Point::out_of_range);
        let to = road.to.unwrap_or_else(Point::out_of_range);

        let w = (self.width() + 1) as f64;
        let h = (self.height() + 1) as f64;

        from.pos[0] >= 0. && from.pos[0] < w && from.pos[1] >= 0. && from.pos[1] < h &&
        to.pos[0] >= 0. && to.pos[0] < w && to.pos[1] >= 0. && to.pos[1] < h
    }

    // returns (accepted, merged)
    fn accept_local_constraints(&self, road: &mut Road) -> (bool, bool) {
        // out of range
        if !self.is_in_range(road) {
            return (false, false);
        }

        let config = self.config.generation(&road.road_type());

        // merge with nearby
        let mut merged = false;
        let merger = road.to.unwrap();
        if self.kdtree
               .has_neighbor_in_range(&merger, config.merge_range) {
            let nearest = self.kdtree.nearest_search(&merger);

            // println!("Merging");

            // not self
            if nearest != merger && nearest != road.from.unwrap() {
                road.set_to(nearest);
                merged = true;
            }
        }

        (true, merged)
    }

    fn propose_with_global_goals(&self, road: &Road, branch: bool) -> Vec<Road> {

        let mut vec: Vec<Road> = Vec::new();

        rules::propose_roads(self.config.generation(&road.road_type()),
                             road,
                             branch,
                             &mut vec);

        let range = self.config.generation(&road.road_type()).fuel_range;

        if branch {
            let mut rng = thread_rng();
            for r in &mut vec {
                let fuel = rng.gen_range(range[0], range[1]);
                r.set_fuel(fuel);
            }
        } else {
            for r in &mut vec {
                r.set_fuel(road.fuel());
            }
        }

        vec
    }
}
