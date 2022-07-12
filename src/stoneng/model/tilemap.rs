use std::collections::HashMap;

use specs::Entity;

use crate::model::spritesheet::SpriteSheet;

use super::spritesheet::SpriteSchema;

/// A tilemap container to manage the world in a 2D game.
///
/// The tiles must still be fed into the ECS, however, this provides a convenient
/// way to build, access and modify the tiles.
pub struct Tilemap {
    pub tiles:          Vec<Entity>
}
impl Tilemap {
    pub fn new(spritesheet: SpriteSheet) -> Self {
        Self { tiles: Vec::new() }
    }
    
    /// Builds a Tilemap from a string and a char->sprite hashmap
    ///
    /// This is used to provide a (de)serializable format for levels
    /// and allows for text-based level building tooling.
    pub fn populate_from_string(&mut self, level: String, atlas_map: HashMap<char, SpriteSchema>) {
        if level.is_empty() {return;}

        let map: Vec<Vec<SpriteSchema>> = vec![];
        
        let mut level_width = 0;
        for line in level.lines() {
            // Ensure standard line width on level
            if level_width != line.len() {
                if level_width == 0 {
                    level_width = line.len() 
                } else {
                    // Malformed level
                }
            }

            
        }
    }
}

