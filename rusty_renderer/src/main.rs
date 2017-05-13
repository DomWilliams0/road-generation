extern crate rusty_roads;

fn main() {
  let s = rusty_roads::Settings {
    width: 960,
    height: 600,
  };

  let r = rusty_roads::generate(&s);
}
