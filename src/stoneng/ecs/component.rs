use specs::{Component, VecStorage, DenseVecStorage};
use std::sync::Arc;
use crate::spritesheet::{SpriteSheet, SpriteSchema, AnimationSchema};
use crate::renderer::RenderSprite;

#[repr(C)]
#[derive(Debug, Component, Clone, Copy, Default)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Into<glm::Vec3> for Position {
    fn into(self) -> glm::Vec3 {
        glm::Vec3::from([self.x, self.y, self.z])
    }
}
impl Into<(f32, f32, f32)> for Position {
    fn into(self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }
}

#[repr(C)]
#[derive(Debug, Component, Clone, Copy)]
#[storage(VecStorage)]
pub struct Scale {
    pub x: f32,
    pub y: f32,
}
impl Into<(f32,f32)> for Scale {
    fn into(self) -> (f32,f32) {
        (self.x, self.y)
    }
}
impl Into<glm::Vec2> for Scale{
    fn into(self) -> glm::Vec2 {
        glm::Vec2::from([self.x, self.y])
    }
}
impl Default for Scale { fn default() -> Self { Self {x: 1.0, y: 1.0} } }

#[repr(C)]
#[derive(Debug, Component, Clone, Copy, Default)]
#[storage(VecStorage)]
pub struct Transform {
    pub translation:    Position,
    pub scale:          Scale,
    pub rotation:       f32,
}

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

#[repr(C)]
#[derive(Debug, Component, Clone)]
#[storage(VecStorage)]
pub struct Sprite {
    pub id:     u16,
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
impl From<(&Sprite, &Transform, &Color)> for RenderSprite {
    fn from(data: (&Sprite, &Transform, &Color)) -> Self {
        let (s, t, c) = data;
        Self {
            translation: t.translation.into(),
            scale:       t.scale.into(),
            rotation:    t.rotation,
            
            color:       c.clone().into(),

            sprite_id:      s.id,
            sprite_dims:    s.dims,
            sprite_flags:   s.flags,
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

#[derive(Debug, Component, Clone, Copy)]
#[storage(DenseVecStorage)]
pub struct PointLight {
    pub intensity: f32,
}


