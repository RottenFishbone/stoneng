mod game;

use stoneng::Config;
use game::RustyLantern;

fn main() {
    let config = Config::default(); 
    stoneng::start(config, move || {
        RustyLantern::new()
    });
}
