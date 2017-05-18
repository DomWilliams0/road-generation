extern crate rusty_roads;
extern crate sfml;

use std::process;
use std::env;
use rusty_roads::{RoadError, RoadMap};

use sfml::system::*;
use sfml::window::{ContextSettings, VideoMode, Event, style, Key};
use sfml::graphics::*;

enum Action {
    GenerateOnly,
    Window,
    Image,
}

fn parse_args() -> Result<Action, RoadError> {
    let mut args = env::args();

    if args.nth(2).is_some() {
        println!("Expected optional single argument of 'generate' or 'image'");
        return Err(RoadError::Args(String::from("Too many args")));
    }

    match env::args().nth(1) {
        None => Ok(Action::Window),
        Some(what) => {
            match &*what {
                "generate" => Ok(Action::GenerateOnly),
                "image" => Ok(Action::Image),
                uhoh => Err(RoadError::Args(format!("Unknown argument {}", uhoh))),
            }
        }
    }
}

fn main() {

    match run() {
        Err(err) => {
            println!("Error: {:?}", err);
            process::exit(1);
        }

        Ok(_) => process::exit(0),
    }

}

fn create_generator() -> Result<RoadMap, RoadError> {
    generate_with_increment(Some(4))
}

fn create_generated() -> Result<RoadMap, RoadError> {
    let mut roadmap = generate_with_increment(None)?;
    roadmap.advance()?;
    Ok(roadmap)
}

fn generate_with_increment(increment: Option<i32>) -> Result<RoadMap, RoadError> {
    println!("Generating roadmap...");
    rusty_roads::RoadmapBuilder::new()
        .size(960, 600)
        .increment(increment)
        .create()
}

fn run() -> Result<(), RoadError> {

    match parse_args()? {
        Action::GenerateOnly => {
            let _roadmap = create_generated()?;
            Ok(())
        }
        Action::Image => render_to_image(),
        Action::Window => open_window(),
    }

}

fn open_window() -> Result<(), RoadError> {

    let mut window = RenderWindow::new(VideoMode::new(960, 600, 32),
                                       "Roads",
                                       style::CLOSE,
                                       &ContextSettings::default())
            .unwrap();

    let mut running = true;
    let mut roadmap = create_generator()?;
    while running {
        for event in window.events() {
            match event {
                Event::Closed => running = false,
                Event::KeyPressed { code, .. } => {
                    match code {
                        Key::Escape => running = false,
                        Key::Space => roadmap = create_generator()?,
                        _ => (),
                    }
                }
                _ => (),
            }
        }

        window.clear(&Color::white());

        roadmap.advance()?;
        render_roadmap(&mut window, &roadmap);
        window.display();
    }

    Ok(())


}

// convenience
#[inline]
fn vec(x: f64, y: f64) -> Vector2f {
    Vector2f::new(x as f32, y as f32)
}

fn render_roadmap(target: &mut RenderTarget, roadmap: &RoadMap) {
    // TODO lazy_static for constants?
    let BACKGROUND_COLOUR: Color = Color::rgb(240, 240, 255);
    let VERTEX_COLOUR: Color = Color::rgba(70, 200, 150, 150);
    let ROAD_COLOUR: Color = Color::rgb(20, 40, 60);

    // cache this
    let WIDTH = roadmap.width() as f64;
    let HEIGHT = roadmap.height() as f64;
    let background = [Vertex::with_pos_color(vec(0.0, 0.0), BACKGROUND_COLOUR),
                      Vertex::with_pos_color(vec(WIDTH, 0.0), BACKGROUND_COLOUR),
                      Vertex::with_pos_color(vec(WIDTH, HEIGHT), BACKGROUND_COLOUR),
                      Vertex::with_pos_color(vec(0.0, HEIGHT), BACKGROUND_COLOUR)];

    target.draw_primitives(&background, PrimitiveType::Quads, RenderStates::default());


    let mut circle = CircleShape::new_init(2.0, 20);
    let rad = circle.radius() as f64;
    circle.set_fill_color(&VERTEX_COLOUR);

    let roads = roadmap.roads();
    for road in roads.iter() {

        if let (Some(from), Some(to)) = road.points() {

            for point in [from, to].iter() {
                circle.set_position(&vec(point.x() - rad, point.y() - rad));
                target.draw(&circle);
            }


            let line = [Vertex::with_pos_color(vec(from.x(), from.y()), ROAD_COLOUR),
                        Vertex::with_pos_color(vec(to.x(), to.y()), ROAD_COLOUR)];

            target.draw_primitives(&line, PrimitiveType::Lines, RenderStates::default());
        }
    }
}


fn render_to_image() -> Result<(), RoadError> {
    let roadmap = create_generated()?;

    let mut texture = RenderTexture::new(roadmap.width() as u32, roadmap.height() as u32, false)
        .ok_or(RoadError::Unknown("Texture creation"))?;

    render_roadmap(&mut texture, &roadmap);

    let path = "/tmp/roadmap.png";
    println!("Saving to '{}'", path);
    match texture
              .texture()
              .copy_to_image()
              .ok_or(RoadError::Unknown("Converting texture to image"))?
              .save_to_file(path) {
        true => Ok(()),
        false => Err(RoadError::Unknown("Saving to file")),
    }
}
