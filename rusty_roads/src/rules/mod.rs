use {Road, RoadType, Point};
use cgmath::{Vector2, Point2, Angle, Rad};
use cgmath::prelude::*;
use config;

mod grid;
mod organic;

enum GenerationRule {
    Grid,
    // Radial,
    Organic,
}

const MAX_PROPOSALS: usize = 3;
type Proposals = [Option<Proposal>; MAX_PROPOSALS];
macro_rules! new_proposals {
  () => {
    [None; MAX_PROPOSALS];
  }
}

type ProposalGenerator = fn(&Point2<f64>, f64, RoadType, bool, f64, f64, &mut Proposals);

#[derive(Copy, Clone)]
pub struct Proposal {
    pub road_type: RoadType,
    pub angle: f64,
    pub from: Point2<f64>,
    pub length: f64,
}
impl Proposal {
    fn to_road(self) -> Road {
        let angle = Rad(self.angle);
        let new_x = self.from.x + (Angle::cos(angle) * self.length);
        let new_y = self.from.y + (Angle::sin(angle) * self.length);

        let arse: Rad<f64> = Rad(20.0f64) + Rad(10.);
        Angle::cos(arse);

        Road::new_with_points(self.road_type,
                              Point::new(self.from.x, self.from.y),
                              Point::new(new_x, new_y))

    }
}
pub fn propose_roads(config: &config::GenerationConfig,
                     road: &Road,
                     branch: bool,
                     out: &mut Vec<Road>) {

    let from = Point2::from(road.from().unwrap().pos);
    let to = Point2::from(road.to().unwrap().pos);

    let vec = (to - from).normalize();
    let Rad(cur_angle) = Rad::atan2(vec.y, vec.x);


    let mut proposals: Proposals = new_proposals!();

    let rule = get_rule(road.to().as_ref().unwrap());
    if let Some(generator) = get_generator(&rule) {
        (generator)(&to,
                    cur_angle,
                    road.road_type(),
                    branch,
                    config.road_chance,
                    config.road_length,
                    &mut proposals);
    }

    for p in proposals
            .iter()
            .take_while(|p| p.is_some())
            .map(|p| p.unwrap()) {
        out.push(p.to_road());
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

fn get_generator(rule: &GenerationRule) -> Option<ProposalGenerator> {
    match *rule {
        GenerationRule::Grid => Some(grid::propose_branching_roads),
        // GenerationRule::Organic => Some(organic::propose_branching_roads), // temporary
        _ => None,
    }
}
