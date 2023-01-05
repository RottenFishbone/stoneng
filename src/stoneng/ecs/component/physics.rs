use specs::{Component, DenseVecStorage};

#[allow(dead_code)]
#[derive(Debug, Component, Clone)]
#[storage(DenseVecStorage)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
impl Velocity {
    pub fn new(x: f32, y: f32) -> Self { Self { x, y } }
}

#[allow(dead_code)]
#[derive(Debug, Component, Copy, Clone)]
#[storage(DenseVecStorage)]
pub struct DampedMoveTarget {
    /// The position to move to 
    pub target:     (f32, f32),

    /// Parameters for movement
    pub frequency:  f32,
    pub damping:    f32,
    pub response:   f32,

    /// constants calculated from the parameters
    constants:      (f32, f32, f32),
    /// State stored for semi-implicit Euler method
    state:          (f32, f32),
}
