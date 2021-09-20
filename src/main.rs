mod engine;
mod game;
extern crate nalgebra_glm as glm;
extern crate stb_image as stb;
use engine::Config;

fn main() {
    let config = Config::default(); 
    let mut game = game::RustyLantern::new();
    engine::start(config, &mut game);
}
