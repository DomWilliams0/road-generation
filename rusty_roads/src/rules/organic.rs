use {Road, RoadType, Point};
use rand::{Rng, thread_rng, random, Closed01};
use std::f64::consts::PI;
use cgmath::{Vector2, Point2, Rad};
use cgmath::prelude::*;

pub fn propose_branching_roads(point: &Point2<f64>,
                               direction: &Vector2<f64>,
                               road_type: RoadType,
                               branch: bool,
                               road_chance: f64,
                               road_length: f64,
                               out: &mut Vec<Road>) {
    const GRID_ANGLES: [f64; 3] = [-PI / 2., 0., PI / 2.];

    let cur_angle: Rad<f64> = Rad::atan2(direction.y, direction.x);


    if !branch {
        out.push(propose_road(GRID_ANGLES[1], cur_angle, road_length, point, road_type));
    } else {

        for grid_angle in &GRID_ANGLES {

            // unlucky
            let Closed01(chance) = random::<Closed01<f64>>();
            if chance > road_chance {
                continue;
            }

            out.push(propose_road(*grid_angle, cur_angle, road_length, point, road_type));
        }
    }

}
fn propose_road(angle: f64,
                cur_angle: Rad<f64>,
                length: f64,
                point: &Point2<f64>,
                road_type: RoadType)
                -> Road {


    // vary angle
    let mut rng = thread_rng();
    const VARIATION: f64 = PI / 6.0; // 30 degrees
    let variation = rng.gen_range(-VARIATION, VARIATION);

    let new_angle = cur_angle + Rad(angle) + Rad(variation);

    let new_x = point.x + (Angle::cos(new_angle) * length);
    let new_y = point.y + (Angle::sin(new_angle) * length);

    Road::new_with_points(road_type,
                          Point::new(point.x, point.y),
                          Point::new(new_x, new_y))
}
