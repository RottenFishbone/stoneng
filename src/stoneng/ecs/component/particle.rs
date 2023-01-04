use specs::{Component, DenseVecStorage};

/// Gives a decrementing lifetime to an entity (in seconds)
/// Once the time reaches 0 the entity will be removed from the world
/// by the particle system.
#[derive(Debug, Component, Clone)]
#[storage(DenseVecStorage)]
pub struct Lifetime {
    pub remaining: f64,
}

/// Applies the scaling factor per second to the paricle.
/// Threshhold can optionally be used to delete the particle below a threshhold.
#[derive(Debug, Component, Clone)]
pub struct Scaling {
    pub factor: f32,
    pub threshold: f32
}
impl Scaling {
    pub fn new(factor: f32) -> Self {Self{factor, threshold: -1.0}}
    pub fn with_thresh(factor: f32, threshold: f32) -> Self {
        Self{factor, threshold}
    }
}

/// Wandering allows for particles to 'float' randomly
/// Bias is a value from -1.0 to 1.0 in which the random acceleration
/// will favour.
/// Strength is the speed in which acceleration is applied
/// Resistance is the amount the particle will slow
#[derive(Debug, Component, Clone)]
#[storage(DenseVecStorage)]
pub struct Wandering {
    pub bias: (f32, f32),
    pub strength: f32,
    pub resistance: f32,
}
