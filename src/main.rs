mod game;

use stoneng::Config;
use game::RustyLantern;

fn main() {
    let mut config = Config::default(); 
    config.dimensions = (800, 600);
    stoneng::start(config, move || {
        RustyLantern::new()
    });
}
