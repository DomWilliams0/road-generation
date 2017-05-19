use RoadType;
use rand::{random, Closed01};
use std::f64::consts::PI;
use cgmath::Point2;
use super::{Proposal, Proposals};
use config::GenerationConfig;

pub fn propose(point: &Point2<f64>,
               cur_angle: f64,
               road_type: RoadType,
               branch: bool,
               config: &GenerationConfig,
               out: &mut Proposals) {
    const GRID_ANGLES: [f64; 3] = [-PI / 2., 0., PI / 2.];

    if !branch {
        out[0] = Some(Proposal {
                          road_type: road_type,
                          angle: cur_angle + GRID_ANGLES[1], // straight
                          from: *point,
                          length: config.road_length,
                      });
    } else {

        for (i, grid_angle) in GRID_ANGLES.iter().enumerate() {

            // unlucky
            let Closed01(chance) = random::<Closed01<f64>>();
            if chance > config.road_chance {
                continue;
            }

            out[i] = Some(Proposal {
                              road_type: road_type,
                              angle: cur_angle + grid_angle,
                              from: *point,
                              length: config.road_length,
                          });
        }
    }
}
