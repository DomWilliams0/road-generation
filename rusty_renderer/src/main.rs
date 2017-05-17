extern crate rusty_roads;
extern crate sfml;

use std::process;
use std::env;
use rusty_roads::{RoadError, RoadMap};

use sfml::system::Vector2f;
use sfml::window::{ContextSettings, VideoMode, Event, style};
use sfml::graphics::*;


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

fn render_roadmap(window: &mut RenderWindow, roadmap: &RoadMap) {
    // TODO lazy_static for constants?
    let BACKGROUND_COLOUR: Color = Color::rgb(200, 200, 210);
    let VERTEX_COLOUR: Color = Color::rgb(100, 200, 100);
    let ROAD_COLOUR: Color = Color::rgb(20, 40, 60);

    // cache this
    let background = [Vertex::with_pos_color(Vector2f::new(0.0, 0.0), BACKGROUND_COLOUR),
                      Vertex::with_pos_color(Vector2f::new(roadmap.width() as f32, 0.0),
                                             BACKGROUND_COLOUR),
                      Vertex::with_pos_color(Vector2f::new(roadmap.width() as f32,
                                                           roadmap.height() as f32),
                                             BACKGROUND_COLOUR),
                      Vertex::with_pos_color(Vector2f::new(0.0, roadmap.height() as f32),
                                             BACKGROUND_COLOUR)];
    window.draw_primitives(&background, PrimitiveType::Quads, RenderStates::default());

    let roads = roadmap.roads();
    for road in roads.iter() {

        if let (Some(from), Some(to)) = road.points() {
            // TODO possible to use own f64 vector?
            let line = [Vertex::with_pos_color(Vector2f::new(from.x() as f32, from.y() as f32),
                                               ROAD_COLOUR),
                        Vertex::with_pos_color(Vector2f::new(to.x() as f32, to.y() as f32),
                                               ROAD_COLOUR)];

            window.draw_primitives(&line, PrimitiveType::Lines, RenderStates::default());
        }
    }

}

fn render(roadmap: &RoadMap) -> Result<(), RoadError> {
    let mut window = RenderWindow::new(VideoMode::new(960, 600, 32),
                                       "Roads",
                                       style::CLOSE,
                                       &ContextSettings::default())
                                       .unwrap();

    loop {
        for event in window.events() {
            if let Event::Closed = event {
                break;
            }
        }

        window.clear(&Color::white());
        render_roadmap(&mut window, roadmap);
        window.display()
    }
}
