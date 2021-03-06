extern crate road_generation;
extern crate sfml;

#[macro_use]
extern crate lazy_static;

use std::process;
use std::env;
use std::fs;
use std::path;
use road_generation::{RoadError, RoadMap, Config};

use sfml::system::*;
use sfml::window::{ContextSettings, VideoMode, Event, style, Key};
use sfml::graphics::*;

const CONFIG_PATH: &'static str = "config.toml";

lazy_static! {
static ref ROAD_RENDERING: [(Color, u32); 3] = [
(Color::rgb(0, 0, 255), 1), // small
(Color::rgb(0, 0, 0), 3), // medium
(Color::rgb(255, 0, 0), 5), // large
];
}

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

fn create_generator(config: &Config) -> Result<RoadMap, RoadError> {
    let (_, new_config) = update_config(Some(config.clone()));
    RoadMap::new(new_config)
}

fn create_generated() -> Result<RoadMap, RoadError> {
    let mut config = load_initial_config()?;
    config.window.growth_increment = None; // instant generation

    let mut roadmap = RoadMap::new(config)?;
    roadmap.advance()?;
    Ok(roadmap)
}

fn load_initial_config() -> Result<Config, RoadError> {
    match update_config(None) {
        (false, _) => {
            Err(RoadError::Settings(format!("Failed to load config from '{}'", CONFIG_PATH)))
        }
        (true, c) => Ok(c),
    }
}

fn update_config(previous: Option<Config>) -> (bool, Config) {
    previous.unwrap_or_default().load(CONFIG_PATH)
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

    window.set_framerate_limit(60);
    window.set_vertical_sync_enabled(true);

    let config = load_initial_config()?;
    let mut roadmap = create_generator(&config)?;

    let mut running = true;
    let mut last_count = 0;
    while running {
        let mut dirty = false;
        for event in window.events() {
            match event {
                Event::Closed => running = false,
                Event::KeyPressed { code, .. } => {
                    match code {
                        Key::Escape => running = false,
                        Key::Space => roadmap = create_generator(&config)?,
                        _ => (),
                    }
                }
                Event::Resized { .. } => dirty = true,
                _ => (),
            }
        }

        roadmap.advance()?;

        // render only if dirty
        let len = roadmap.roads().len();
        if len != last_count {
            dirty = true;
        }
        last_count = len;


        if dirty {
            window.clear(&Color::white());
            render_roadmap(&mut window, &roadmap);
        }

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
    let background_colour: Color = Color::rgb(240, 240, 255);
    let vertex_colour: Color = Color::rgba(70, 200, 150, 150);

    // cache this
    let width = roadmap.width() as f64;
    let height = roadmap.height() as f64;
    let background = [Vertex::with_pos_color(vec(0.0, 0.0), background_colour),
                      Vertex::with_pos_color(vec(width, 0.0), background_colour),
                      Vertex::with_pos_color(vec(width, height), background_colour),
                      Vertex::with_pos_color(vec(0.0, height), background_colour)];

    target.draw_primitives(&background, PrimitiveType::Quads, RenderStates::default());


    let mut circle = CircleShape::new_init(2.0, 20);
    let rad = circle.radius() as f64;
    circle.set_fill_color(&vertex_colour);

    let roads = roadmap.roads();
    for road in roads.iter() {

        if let (Some(from), Some(to)) = road.points() {

            for point in &[from, to] {
                circle.set_position(&vec(point.x() - rad, point.y() - rad));
                // target.draw(&circle);
            }

            let (colour, _thickness) = ROAD_RENDERING[road.road_type() as usize];

            let line = [Vertex::with_pos_color(vec(from.x(), from.y()), colour),
                        Vertex::with_pos_color(vec(to.x(), to.y()), colour)];

            target.draw_primitives(&line, PrimitiveType::Lines, RenderStates::default());
        }
    }
}

const RENDER_DIR: &'static str = "/tmp/roads";
const RENDER_COUNT: u32 = 10;

fn render_to_image() -> Result<(), RoadError> {

    let _ = fs::remove_dir_all(RENDER_DIR); // ignore error
    fs::DirBuilder::new()
        .create(RENDER_DIR)
        .or(Err(RoadError::Unknown("Render directory creation")))?;
    println!("Generating {} roadmaps in {}", RENDER_COUNT, RENDER_DIR);

    for i in 0..RENDER_COUNT {
        println!("Rendering {}/{}", i + 1, RENDER_COUNT);
        let roadmap = create_generated()?;

        let mut texture =
            RenderTexture::new(roadmap.width() as u32, roadmap.height() as u32, false)
                .ok_or(RoadError::Unknown("Texture creation"))?;

        render_roadmap(&mut texture, &roadmap);
        let path = path::Path::join(path::Path::new(RENDER_DIR), format!("road-{}.png", i));

        if !texture
                .texture()
                .copy_to_image()
                .ok_or(RoadError::Unknown("Converting texture to image"))?
                .save_to_file(path.to_str().unwrap()) {
            return Err(RoadError::Unknown("Saving to file"));
        }

    }

    Ok(())
}
