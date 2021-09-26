mod game;

use stoneng::Config;

fn main() {
    let config = Config::default(); 
    let mut game = game::RustyLantern::new();
    stoneng::start(config, &mut game);
}
