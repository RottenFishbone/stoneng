use glm::{Vec2, Vec3, Vec4};

pub enum SpriteFlag {
    /// Have the renderer animate the sprite
    Animated    = 0b0000001,
    /// Loop the sprite on animation complete
    Looping     = 0b0000010,
    /// Reverse the animation at the end
    Reverse     = 0b0000100,

    /// Used by the renderer to determine if a sprite is reversing.
    /// This can be used mid animation to anim cancel with a rewind.
    Reversing   = 0b1000000,
}

/// A sprite's data representation.
///
/// This data is passed directly to the shader and should be tightly packed.
#[repr(C, packed)]
#[derive(Debug, Copy, Clone, PartialEq)]
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
    pub sprite_id:  u32,
    /// The dimension of the sprite measured in tiles.
    ///
    /// This is NOT the scale of the sprite, rather how many tiles in the atlas
    /// to build the sprite.
    /// The function Sprite::redimension() can be used to alter this after the 
    /// sprite has been created.
    /// Data is packed as two 4-bit values, x being the least significant bits.
    sprite_dims:    u8, 
    /// Flags to modify sprite behaviour
    pub sprite_flags:   u8,

}
 
impl Sprite {
    pub fn new(pos: Vec3, color: Vec4, scale: Vec2,
                rotation: f32, dims: (u8, u8),
                sprite_id: u32) 
                -> Self {
        Self {
            pos, color, scale, rotation, sprite_id,
            sprite_dims: ((dims.1-1) << 4) | (dims.0-1),
            sprite_flags: 0,
        }
    }
    
    pub fn default() -> Self {
        Self { 
            pos: Vec3::default(), 
            color: Vec4::new(1.0, 1.0, 1.0, 1.0),
            scale: Vec2::new(1.0, 1.0),
            rotation: 0.0,
            sprite_id: 0, sprite_dims: 0, sprite_flags: 0,
        }
    }
    
    pub fn check_flag(&mut self, flag: SpriteFlag) -> bool {
        self.sprite_flags & (flag as u8) > 0
    }

    pub fn set_flag(&mut self, flag: SpriteFlag, state: bool){
        if state {
            self.sprite_flags |= flag as u8;
        }
        else {
            self.sprite_flags &= !(flag as u8);
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

impl PartialOrd for Sprite {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let other_z: f32;
        let self_z: f32;
        unsafe {
            let self_pos = std::ptr::addr_of!(self.pos).read_unaligned();
            let other_pos = std::ptr::addr_of!(other.pos).read_unaligned();
            self_z = std::ptr::addr_of!(self_pos.z).read_unaligned();
            other_z = std::ptr::addr_of!(other_pos.z).read_unaligned();
        }

        (&self_z).partial_cmp(&other_z)
    }
}
