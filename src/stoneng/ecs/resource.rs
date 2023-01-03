use shrev::EventChannel;
use specs::Entity;

#[derive(Default, Clone, Debug)]
pub struct SpritesheetPath(pub String);

#[derive(Default, Clone, Copy, Debug)]
pub struct DeltaTime(pub f64);

#[derive(Default, Clone, Copy, Debug)]
pub struct WindowSize(pub f32, pub f32);

#[derive(Default, Clone, Copy, Debug)]
pub struct View(pub f32, pub f32, pub f32);

#[derive(Debug, Clone, Copy)]
pub struct CollisionEvent {
    pub collider_a: Entity,
    pub collider_b: Entity,
}
impl CollisionEvent {
    pub fn new(a: Entity, b: Entity) -> Self {
        Self { collider_a: a, collider_b: b }
    }
}
pub type CollisionEvents = EventChannel<CollisionEvent>;
