extern crate rusty_roads;
extern crate sfml;

use std::process;
use std::env;
use rusty_roads::{RoadError, RoadMap};

use sfml::system::*;
use sfml::window::{ContextSettings, VideoMode, Event, style, Key};
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
    let mut settings = rusty_roads::RoadmapBuilder::new();
    let roadmap = settings.size(960, 600).generate()?;

    if do_render {
        println!("Rendering...");
        render(&roadmap)
    } else {
        Ok(())
    }
}

// convenience
#[inline]
fn vec(x: f64, y: f64) -> Vector2f {
    Vector2f::new(x as f32, y as f32)
}

fn render_roadmap(window: &mut RenderWindow, roadmap: &RoadMap) {
    // TODO lazy_static for constants?
    let BACKGROUND_COLOUR: Color = Color::rgb(200, 200, 210);
    let VERTEX_COLOUR: Color = Color::rgb(200, 100, 150);
    let ROAD_COLOUR: Color = Color::rgb(20, 40, 60);

    // cache this
    let WIDTH = roadmap.width() as f64;
    let HEIGHT = roadmap.height() as f64;
    let background = [Vertex::with_pos_color(vec(0.0, 0.0), BACKGROUND_COLOUR),
                      Vertex::with_pos_color(vec(WIDTH, 0.0), BACKGROUND_COLOUR),
                      Vertex::with_pos_color(vec(WIDTH, HEIGHT), BACKGROUND_COLOUR),
                      Vertex::with_pos_color(vec(0.0, HEIGHT), BACKGROUND_COLOUR)];

    window.draw_primitives(&background, PrimitiveType::Quads, RenderStates::default());


    let mut circle = CircleShape::new_init(2.0, 20);
    let rad = circle.radius() as f64;
    circle.set_fill_color(&VERTEX_COLOUR);

    let roads = roadmap.roads();
    for road in roads.iter() {

        if let (Some(from), Some(to)) = road.points() {

            for point in [from, to].iter() {
                circle.set_position(&vec(point.x() - rad, point.y() - rad));
                window.draw(&circle);
            }


            let line = [Vertex::with_pos_color(vec(from.x(), from.y()), ROAD_COLOUR),
                        Vertex::with_pos_color(vec(to.x(), to.y()), ROAD_COLOUR)];

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

    let mut running = true;
    while running {
        for event in window.events() {
            match event {
                Event::Closed => running = false,
                Event::KeyPressed { code, .. } => {
                    match code {
                        Key::Escape => running = false,
                        Key::Space => (), // TODO regenerate
                        _ => (),
                    }
                }
                _ => (),
            }
        }
        window.clear(&Color::white());
        render_roadmap(&mut window, roadmap);
        window.display();
    }

    Ok(())
}
