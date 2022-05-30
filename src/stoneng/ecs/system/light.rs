use specs::{ReadStorage, WriteStorage, System, Join, Read, SystemData};
use specs::prelude::*;
use std::sync::Arc;
use crate::{
    model::spritesheet::{SpriteSheet, AnimationSchema},
    ecs::resource::{DeltaTime, WindowSize},
    ecs::component::{Color, Transform, Sprite, Animation, PointLight},
    renderer::{
        sprite::{RenderSprite, SpriteRenderer}, 
        light::{RenderLight, LightRenderer},
    },
};


#[derive(Default)] 
pub struct LightRenderSys {
    renderer: LightRenderer,
}
impl<'a> System<'a> for LightRenderSys {
    type SystemData = (ReadStorage<'a, Transform>,
                       ReadStorage<'a, PointLight>,
                       Read<'a, WindowSize>);

    fn run(&mut self, data: Self::SystemData) {
        let (xforms, lights, window) = data;
        let window = (window.0, window.1);

        let lights: Vec<RenderLight> = (&xforms, &lights).join()
            .map(|data| data.into())
            .collect();

        self.renderer.render(&lights, window);
    }
    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.renderer = LightRenderer::new();
        self.renderer.init().unwrap();
        self.renderer.dither_scale = 2.0;
    }
}
