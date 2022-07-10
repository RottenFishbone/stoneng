use specs::{Component, VecStorage, DenseVecStorage};

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
pub struct Rotation {
    pub deg: f32,
}
