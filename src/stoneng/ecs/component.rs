use specs::{Component, VecStorage, DenseVecStorage};
use std::sync::Arc;
use crate::model::spritesheet::{SpriteSheet, SpriteSchema, AnimationSchema};
use crate::renderer::{
    sprite::RenderSprite, 
    light::RenderLight,
    text::RenderString,
};

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

/// A Sprite component is a renderable sub-texture from a SpriteSys' atlas
///
/// These are easily convertable into a RenderSprite which is used by OpenGL
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
    /// Builds a the struct used to render a sprite from it's requisite components
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
impl From<Option<&Arc<AnimationSchema>>> for Animation {
    /// Creates an Animation Component using an AnimationSchema reference
    fn from(schema: Option<&Arc<AnimationSchema>>) -> Self {
        // Try to spawn a new Option-wrapped Arc
        let idle_anim = match schema {
            Some(anim) => Some(anim.clone()),
            None => None,
        };

        Self {
            frame: 0,
            frame_progress: 0.0,
            is_reversing: false,
            schema: idle_anim,
        }
    }
}

#[derive(Debug, Component, Clone, Copy)]
#[storage(DenseVecStorage)]
pub struct PointLight {
    pub intensity: f32,
}
impl From<(&Transform, &PointLight)> for RenderLight {
    fn from(data: (&Transform, &PointLight)) -> Self {
        let (t, p) = data;
        Self {
            pos: (t.translation.x, t.translation.y),
            intensity: p.intensity
        }
    }
}


#[derive(Debug, Component, Clone)]
#[storage(DenseVecStorage)]
pub struct Text {
    pub content: String,
    pub size: f32,
    pub offset: (f32, f32), // TODO implement
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
            position:    p.clone().into(),
            size:        t.size,
            color:       c.clone().into(),
            
            text:        t.content.clone(),
        }
    }
}

#[derive(Debug, Component, Clone)]
#[storage(DenseVecStorage)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

