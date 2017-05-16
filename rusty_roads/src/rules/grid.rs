use super::super::RoadType; // oof
use generator::{Road, Point};
use rand::{Rng, thread_rng, random, Closed01};
use std::f64::consts::PI;
use cgmath::{Vector2, Point2, Rad};
use cgmath::prelude::*;

pub fn propose_branching_roads(point: &Point2<f64>,
                               direction: &Vector2<f64>,
                               road_type: RoadType,
                               out: &mut Vec<Road>) {
    const ROAD_CHANCE: f64 = 0.8;
    const ROAD_LENGTH: f64 = 20.;
    const GRID_ANGLES: [f64; 3] = [-PI / 2., 0., PI / 2.];

    let cur_angle: Rad<f64> = Rad::atan2(direction.y, direction.x);

    // let mut rng = thread_rng();
    for grid_angle in GRID_ANGLES.iter() {

        // unlucky
        let Closed01(chance) = random::<Closed01<f64>>();
        if chance > ROAD_CHANCE {
            continue;
        }

        let new_angle = cur_angle + Rad(grid_angle.clone());

        let new_x = point.x + (Angle::cos(new_angle) * ROAD_LENGTH);
        let new_y = point.y + (Angle::sin(new_angle) * ROAD_LENGTH);

        let new_road = Road::new_with_points(road_type,
                                             Point::new(point.x, point.y),
                                             Point::new(new_x, new_y));
        out.push(new_road);
    }

}
