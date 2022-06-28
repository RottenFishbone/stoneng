use specs::{ReadStorage, WriteStorage, System, Join, Read, SystemData};
use specs::prelude::*;
use crate::ecs::{
    component::{Transform, Position, Velocity},
    resource::DeltaTime,
};

#[derive(Default)]
pub struct MovementSys;
impl<'a> System<'a> for MovementSys {
    type SystemData = (ReadStorage<'a, Velocity>,
                       WriteStorage<'a, Transform>,
                       WriteStorage<'a, Position>,
                       Read<'a, DeltaTime>);

    fn run(&mut self, data: Self::SystemData) {
        let (vels, mut xforms, mut positions, dt) = data;
        let dt = dt.0 as f32;

        for (vel, xform) in (&vels, &mut xforms).join() {
            xform.translation.x += vel.x * dt;
            xform.translation.y += vel.y * dt;
        }

        for (vel, pos) in (&vels, &mut positions).join() {
            pos.x += vel.x * dt;
            pos.y += vel.y * dt;
        }
    }
}
