pub mod transform;
pub mod sprite;
pub mod physics;
pub mod tile;
pub mod collision;
pub mod particle; 

use specs::{Component, DenseVecStorage};
use crate::renderer::{
    light::RenderLight,
    text::RenderString,
};

pub use transform::Position as Position;
pub use transform::Scale as Scale;
pub use transform::Rotation as Rotation;

pub use sprite::Color as Color;
pub use sprite::Sprite as Sprite;
pub use sprite::Animation as Animation;

pub use physics::Velocity as Velocity;
 
pub use tile::Tile as Tile;
pub use tile::Floor as Floor;
pub use tile::Wall as Wall;

pub use particle::Lifetime as Lifetime;
pub use particle::Wandering as Wandering;
pub use particle::Scaling as Scaling;

pub use collision::Collider as Collider;

#[derive(Debug, Component, Clone, Copy)]
#[storage(DenseVecStorage)]
pub struct PointLight {
    pub intensity: f32,
    pub scaled: bool,
}
impl PointLight {
    pub fn new(intensity: f32) -> Self { Self { intensity, scaled: false } }
    pub fn new_scaled(intensity: f32) -> Self { Self { intensity, scaled: true } }
}
impl From<(&Position, &PointLight)> for RenderLight {
    fn from(data: (&Position, &PointLight)) -> Self {
        let (p, l) = data;
        Self {
            pos: (p.x, p.y),
            intensity: l.intensity
        }
    }
}

#[derive(Debug, Component, Clone)]
#[storage(DenseVecStorage)]
pub struct Text {
    pub content: String,
    pub size: f32,
    pub offset: (f32, f32),
}
impl Text {
    pub fn new(content: String, size: f32, offset: (f32, f32)) -> Self {
        Self { content, size, offset }
    }
}
impl From<String> for Text {
    fn from(content: String) -> Self {
        Self { content, size: 1.0, offset: (0.0, 0.0) }
    }
}
impl From<&str> for Text {
    fn from(content_slice: &str) -> Self {
        Self { content: String::from(content_slice), size: 1.0, offset: (0.0, 0.0) }
    }
}
impl From<(&Text, &Position, &Color)> for RenderString {
    /// Builds a the struct used to render a sprite from it's requisite components
    fn from(data: (&Text, &Position, &Color)) -> Self {
        let (t, p, c) = data;
        Self {
            position:    (p.x + t.offset.0, p.y + t.offset.1, p.z),
            size:        t.size,
            color:       c.clone().into(),
            
            text:        t.content.clone(),
        }
    }
}


