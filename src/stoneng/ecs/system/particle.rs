use specs::{ReadStorage, WriteStorage, System, Join, Read, SystemData};
use specs::prelude::*;
use crate::{
    ecs::component::{Lifetime, Wandering, Velocity, Scaling, Scale},
    ecs::resource::DeltaTime,
};

use rand::Rng;

pub struct ParticleSys;
impl<'a> System<'a> for ParticleSys {
    type SystemData = (Entities<'a>,
                       WriteStorage<'a, Lifetime>,
                       ReadStorage<'a, Wandering>,
                       WriteStorage<'a, Velocity>,
                       ReadStorage<'a, Scaling>,
                       WriteStorage<'a, Scale>,
                       Read<'a, DeltaTime>);

    fn run(&mut self, data: Self::SystemData) {
        let (ents, mut lifetimes, wanderers, mut vels, 
            scalings, mut scales, dt) = data;
        let dt = dt.0;
        
        // Lifetimes
        for (ent, life) in (&ents, &mut lifetimes).join() {
            life.remaining -= dt;

            if life.remaining <= 0.0 {
                let _ = ents.delete(ent);
            }
        }

        // Wandering
        // TODO implement
        for (wander, vel) in (&wanderers, &mut vels).join() {}
    
        // Scaling
        let time_factor = 1.0 - dt as f32;
        for (ent, scaling, scale) in (&ents, &scalings, &mut scales).join() {
            scale.x *= scaling.factor * time_factor;
            scale.y *= scaling.factor * time_factor;

            if scale.x.abs() < scaling.threshold || 
               scale.y.abs() < scaling.threshold {
                
                let _ = ents.delete(ent);
            }
        }
    }
}
