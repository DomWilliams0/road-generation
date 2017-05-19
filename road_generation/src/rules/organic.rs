use RoadType;
use rand::{thread_rng, Rng};
use cgmath::Point2;
use super::{Proposals, grid};
use config::GenerationConfig;

pub fn propose(point: &Point2<f64>,
               cur_angle: f64,
               road_type: RoadType,
               branch: bool,
               config: &GenerationConfig,
               out: &mut Proposals) {
    grid::propose(point,
                  cur_angle,
                  road_type,
                  branch,
                  config,
                  out);


    // vary grid angle
    let mut rng = thread_rng();
    let variation = config.organic_angle.to_radians();

    for prop in out.iter_mut().take_while(|p| p.is_some()) {
        let variation = rng.gen_range(-variation, variation);
        *prop = prop.map(|mut p| {
                             p.angle += variation;
                             p
                         });
    }

}
