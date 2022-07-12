use std::sync::Arc;
use specs::{Component, DenseVecStorage};

use crate::{
    model::spritesheet::SpriteSchema, 
    renderer::sprite::RenderSprite
};

#[derive(Debug, Component, Clone)]
#[storage(DenseVecStorage)]
pub struct Tile {
    pub pos: (i32, i32),
}

#[derive(Debug, Component, Clone)]
#[storage(DenseVecStorage)]
pub struct Floor {
    pub schema: Arc<SpriteSchema>,
}

#[derive(Debug, Component, Clone)]
#[storage(DenseVecStorage)]
pub struct Wall {
    pub schema: Arc<SpriteSchema>,
}

// Sorry for this...
impl From<(
        &Tile, 
        &crate::ecs::component::Color, 
        Arc<SpriteSchema>, 
        (f32, f32), 
        f32
    )> for RenderSprite {
    
    /// Convert from a packed set of relevant tile data into a RenderSprite
    ///
    /// Tile, Color, SpriteSchema, Scale, Z-order
    fn from(data: 
            (&Tile, 
             &crate::ecs::component::Color, 
             Arc<SpriteSchema>, 
             (f32, f32), 
             f32)) -> Self {

        let (tile, color, schema, scale, z) = data;
        let pos = (tile.pos.0 as f32 * scale.0 * 10.0, tile.pos.1 as f32 * scale.1 * 10.0);
        RenderSprite {
            translation:    (pos.0, pos.1, z),
            scale,
            rotation:       0.0,

            color:          color.clone().into(),

            sprite_id:      schema.root,
            sprite_dims:    0,
            sprite_flags:   0,
            reserved:       0,
        }
    }
}

