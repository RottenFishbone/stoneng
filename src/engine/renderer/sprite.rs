use glm::{Vec2, Vec3, Vec4};

pub enum SpriteFlags {
    
}

/// A sprite's data representation.
///
/// This data is passed directly to the shader and should be tightly packed.
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Sprite {
    /// A 3d position of the sprite
    pub pos:        Vec3,
    /// A 4d color tint for the sprite, RGBA
    pub color:      Vec4,
    /// The scale applied to the sprite. This will stretch/shrink the image.
    pub scale:      Vec2,
    /// The 360 deg rotation of the sprite
    pub rotation:   f32,

    /// The position of the sprite in the atlas
    pub sprite_id:  u16,
    /// Flags to modify sprite behaviour
    sprite_flags:   u8,
    /// The dimension of the sprite measured in tiles.
    ///
    /// This is NOT the scale of the sprite, rather how many tiles in the atlas
    /// to build the sprite.
    /// The function Sprite::redimension() can be used to alter this after the 
    /// sprite has been created.
    /// Data is packed as two 4-bit values, x being the least significant bits.
    sprite_dims:    u8,

    /// The current animation for the sprite, 0 being idle.
    pub anim_id:        u8,
    /// The current from of the animation
    pub anim_frame:     u8,
    /// The total frames of the animation
    pub anim_total:     u8,
    /// How long to wait between frames. Used to slow an animation.
    pub anim_wait:      u8,
}

impl Sprite {
    pub fn new(pos: Vec3, color: Vec4, scale: Vec2,
                rotation: f32, dims: (u8, u8),
                sprite_id: u16) 
                -> Self {
        Self {
            pos, color, scale, rotation, sprite_id,
            sprite_dims: ((dims.1-1) << 4) | (dims.0-1),
            sprite_flags: 0,
            anim_id: 0, anim_frame: 0, anim_total: 0, anim_wait: 0,
        }
    }
    
    pub fn default() -> Self {
        Self { 
            pos: Vec3::default(), 
            color: Vec4::new(1.0, 1.0, 1.0, 1.0),
            scale: Vec2::new(1.0, 1.0),
            rotation: 0.0,
            sprite_id: 0, sprite_dims: 0, sprite_flags: 0,
            anim_id: 0, anim_frame: 0, anim_total: 0, anim_wait: 0
        }
    }
    
    /// Changes the sprite dimensions of the sprite.
    ///
    /// This value is how many sprite tiles wide the sprite is.
    /// That is, the tile resolution NOT the scale of the sprite.
    pub fn redimension(&mut self, dims: (u8, u8)) {
        self.sprite_dims = (dims.1-1) << 4 | (dims.0-1); 
    }
}


