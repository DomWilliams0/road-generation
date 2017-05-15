extern crate rusty_roads;
extern crate piston_window;

use std::process;
use std::env;
use rusty_roads::{RoadError, RoadMap};
use piston_window::*;

fn main() {
    let render = env::args().nth(1).map(|x| x != "no-render");

    match run(render.unwrap_or(true)) {
        Err(err) => {
            println!("Error: {:?}", err);
            process::exit(1);
        }

        Ok(_) => process::exit(0),
    }

}

fn run(do_render: bool) -> Result<(), RoadError> {

    println!("Generating roadmap...");
    let roadmap = rusty_roads::RoadmapBuilder::new()
        .size(960, 600)
        .generate()?;

    if do_render {
        println!("Rendering...");
        render(&roadmap)
    } else {
        Ok(())
    }
}

fn render_roadmap(c: &Context, g: &mut G2d, roadmap: &RoadMap) {
    const RADIUS: f64 = 8.;

    rectangle([0.7, 0.7, 0.7, 1.],
              [0., 0., roadmap.width() as f64, roadmap.height() as f64],
              c.transform,
              g);

    let roads = roadmap.roads();
    for road in roads.iter() {

        if let (Some(from), Some(to)) = road.points() {
            for point in [from, to].iter() {
                ellipse([0.2, 0.7, 0.5, 1.],
                        [point.x() - RADIUS / 2.,
                         point.y() - RADIUS / 2.,
                         RADIUS,
                         RADIUS],
                        c.transform,
                        g)
            }


            line([0., 0., 0., 1.],
                 2.,
                 [from.x(), from.y(), to.x(), to.y()],
                 c.transform,
                 g);

        }
    }

}

fn render(roadmap: &RoadMap) -> Result<(), RoadError> {

    let mut window: PistonWindow = WindowSettings::new("Roadmap", [600; 2])
        .exit_on_esc(true)
        .build()
        .expect("Failed to create window");
    window.set_lazy(true);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1., 1., 1., 1.], g);
            render_roadmap(&c, g, roadmap);
        });
    }

    Ok(())
}
