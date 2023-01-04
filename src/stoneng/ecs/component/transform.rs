use specs::{Component, VecStorage, DenseVecStorage};

#[repr(C)]
#[derive(Debug, Component, Clone, Copy, Default)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl From<Position> for glm::Vec3 {
    fn from(p: Position) -> Self { Self::from([p.x, p.y, p.z]) }
}
impl From<Position> for (f32, f32, f32) {
    fn from(p: Position) -> Self { (p.x, p.y, p.z) }
}
impl From<(f32,f32,f32)> for Position {
    fn from(v: (f32,f32,f32)) -> Self { Self {x: v.0, y: v.1, z: v.2} }
}

#[repr(C)]
#[derive(Debug, Component, Clone, Copy)]
#[storage(VecStorage)]
pub struct Scale {
    pub x: f32,
    pub y: f32,
}
// TODO change to From<Scale>
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
