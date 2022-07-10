use std::sync::Arc;

use specs::{Component, VecStorage, DenseVecStorage};

use crate::{
    ecs::component::transform::{Scale, Position, Rotation},
    model::spritesheet::{SpriteSheet, SpriteSchema, AnimationSchema},
    renderer::sprite::RenderSprite,
};

#[repr(C)]
#[derive(Debug, Component, Clone, Copy)]
#[storage(VecStorage)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl Into<(f32, f32, f32, f32)> for Color {
    fn into(self) -> (f32, f32, f32, f32) {
        (self.r, self.g, self.b, self.a)
    }
}
impl Default for Color { fn default() -> Self { Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 } } }

/// A Sprite component is a renderable sub-texture from a SpriteSys' atlas
///
/// These are easily convertable into a RenderSprite which is used by OpenGL
#[repr(C)]
#[derive(Debug, Component, Clone)]
#[storage(VecStorage)]
pub struct Sprite {
    pub id:     u32,
    pub dims:   u8,
    pub flags:  u8,
    pub schema: Arc<SpriteSchema>,
}
impl From<Arc<SpriteSchema>> for Sprite {
    fn from(schema: Arc<SpriteSchema>) -> Self {
        Self {
            id: 0, dims: 0, flags: 0,
            schema: schema.clone()
        }
    }
}
impl From<(&Sprite, &Position, &Scale, &Color)> for RenderSprite {
    /// Builds a the struct used to render a sprite from it's requisite components
    fn from(data: (&Sprite, &Position, &Scale, &Color)) -> Self {
        let (spr, p, s, c) = data;
        Self {
            translation: p.clone().into(),
            scale:       s.clone().into(),
            rotation:    0.0,

            color:       c.clone().into(),

            sprite_id:      spr.id,
            sprite_dims:    spr.dims,
            sprite_flags:   spr.flags,
            reserved:       0,
        }
    }
}

#[derive(Debug, Component, Clone)]
#[storage(DenseVecStorage)]
pub struct Animation {
    pub frame:          u8,
    pub frame_progress: f32,
    pub is_reversing:   bool,
    pub schema:         Option<Arc<AnimationSchema>>,
}
impl Animation {
    pub fn from_name(name: &str, sprite: &Sprite) -> Self {
        sprite.schema.animations.get(name).into()
    }
}
impl From<Option<&Arc<AnimationSchema>>> for Animation {
    /// Creates an Animation Component using an AnimationSchema reference
    fn from(schema: Option<&Arc<AnimationSchema>>) -> Self {
        // Try to spawn a new Option-wrapped Arc
        let anim = match schema {
            Some(anim) => Some(anim.clone()),
            None => None,
        };

        Self {
            frame: 0,
            frame_progress: 0.0,
            is_reversing: false,
            schema: anim,
        }
    }
}
