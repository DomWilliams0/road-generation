use {Road, RoadType, Point};
use rand::{random, Closed01};
use std::f64::consts::PI;
use cgmath::{Vector2, Point2, Rad};
use cgmath::prelude::*;

pub fn propose_branching_roads(point: &Point2<f64>,
                               direction: &Vector2<f64>,
                               road_type: RoadType,
                               branch: bool,
                               out: &mut Vec<Road>) {
    const ROAD_CHANCE: f64 = 0.8;
    const ROAD_LENGTH: f64 = 20.;
    const GRID_ANGLES: [f64; 3] = [-PI / 2., 0., PI / 2.];

    let cur_angle: Rad<f64> = Rad::atan2(direction.y, direction.x);


    if !branch {
        out.push(propose_road(GRID_ANGLES[1], cur_angle, ROAD_LENGTH, point, road_type));
    } else {

        for grid_angle in &GRID_ANGLES {

            // unlucky
            let Closed01(chance) = random::<Closed01<f64>>();
            if chance > ROAD_CHANCE {
                continue;
            }

            out.push(propose_road(*grid_angle, cur_angle, ROAD_LENGTH, point, road_type));
        }
    }

}
fn propose_road(angle: f64,
                cur_angle: Rad<f64>,
                length: f64,
                point: &Point2<f64>,
                road_type: RoadType)
                -> Road {

    let new_angle = cur_angle + Rad(angle);

    let new_x = point.x + (Angle::cos(new_angle) * length);
    let new_y = point.y + (Angle::sin(new_angle) * length);

    Road::new_with_points(road_type,
                          Point::new(point.x, point.y),
                          Point::new(new_x, new_y))
}
