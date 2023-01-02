use specs::{Component, DenseVecStorage, Entity};

#[allow(dead_code)]
#[derive(Debug, Component, Clone)]
#[storage(DenseVecStorage)]
pub struct Collider {
    pub width:  f32,
    pub height: f32,
}
impl Collider {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

