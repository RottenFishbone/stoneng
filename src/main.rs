mod engine;
mod game;

use engine::Config;

fn main() {
    let config = Config {
        dimensions: (800, 600),
        title: "Rusty Lantern".into(),
    };
    let mut game = game::RustyLantern {};
    engine::start(config, &mut game);
}
