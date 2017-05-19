use RoadType;
use rand::{thread_rng, Rng};
use std::f64::consts::PI;
use cgmath::Point2;
use super::{Proposals, grid};

pub fn propose(point: &Point2<f64>,
               cur_angle: f64,
               road_type: RoadType,
               branch: bool,
               road_chance: f64,
               road_length: f64,
               out: &mut Proposals) {

    grid::propose(point,
                  cur_angle,
                  road_type,
                  branch,
                  road_chance,
                  road_length,
                  out);


    // vary grid angle
    const VARIATION: f64 = PI / 6.0; // 30 degrees
    let mut rng = thread_rng();



    for prop in out.iter_mut().take_while(|p| p.is_some()) {
        let variation = rng.gen_range(-VARIATION, VARIATION);
        *prop = prop.map(|mut p| {
                             p.angle += variation;
                             p
                         });
    }

}
