mod game;

use stoneng::Config;
use game::GameState;

fn main() {
    let mut config = Config::default(); 
    config.dimensions = (800, 600);
    stoneng::start(config, move || {
        GameState::new()
    });
}
