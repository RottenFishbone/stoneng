use specs::{ReadStorage, WriteStorage, System, Join, Read, SystemData};
use specs::prelude::*;
use std::sync::Arc;
use crate::{
    spritesheet::{SpriteSheet, AnimationSchema},
    ecs::resource::DeltaTime,
    ecs::component::{Color, Transform, Sprite, Animation},
    renderer::{RenderSprite, SpriteRenderer},
};

#[derive(Default)]
pub struct StaticSpriteSys; 
impl<'a> System<'a> for StaticSpriteSys {
    type SystemData = WriteStorage<'a, Sprite>;

    fn run(&mut self, data: Self::SystemData) {
        let mut sprites = data;
        for mut s in (&mut sprites).join() {
            s.id = s.schema.root;
            let (x, y) = s.schema.dimensions;
            s.dims = x | (y << 4);
        }
    }
}

#[derive(Default)]
pub struct AnimSpriteSys;
impl AnimSpriteSys {
    fn sprite_to_idle(sprite: &mut Sprite, anim: &mut Animation) {
        anim.frame = 0;
        anim.frame_progress = 0.0;
        anim.is_reversing = false;        
        anim.schema = sprite.schema.animations.get("idle").map(|s| s.clone());
    }

    fn advance_animation(s: &mut Sprite, a: &mut Animation, dt: f64){
        let schema = match &a.schema {
            Some(schema) => {schema.clone()},
            None => return,
        };
        
        // Calculate how many frames need to be played ( as a float )
        a.frame_progress += (dt/(schema.frame_time as f64)) as f32;

        // Figure out how many WHOLE frames to advance ( as an int )
        let frames_to_adv = a.frame_progress.floor();
        // Remove those from frame progress
        a.frame_progress -= frames_to_adv;
        
        // Prevent overflow
        if frames_to_adv > 255.0 { 
            s.id = schema.root + (a.frame as u16);
            return; 
        } 

        let mut adv_frames = frames_to_adv as u8;

        while adv_frames > 0 {
             // Try to play forwards
            if !a.is_reversing {
                // Advance all frames, if possible
                if (a.frame as u32 + adv_frames as u32) < schema.frames as u32 {
                    a.frame += adv_frames;
                    break;
                } 
                // Not all frames could be advanced
                else{
                    // Calculate the number of valid frames and advance those
                    let valid_frames = (a.frame + adv_frames) - schema.frames;
                    adv_frames -= valid_frames;
                    a.frame += valid_frames;
                    // Note, there is AT LEAST one frame left now.

                    if schema.reverses { a.is_reversing = true; }
                    else if schema.loops {
                        // Return to the first frame again, costing one frame.
                        adv_frames -= 1;
                        a.frame = 0;
                    }
                    // Return to idle
                    else {
                        Self::sprite_to_idle(s, a);
                        return;
                    }
                }
            }

            // Otherwise, play in reverse
            else {
                // Reverse all the frames, if possible
                if a.frame >= adv_frames {
                    a.frame -= adv_frames;
                    s.id -= adv_frames as u16;
                    break;
                }
                // Not all frames could be reversed
                else {
                    // Reverse back to 0
                    adv_frames -= a.frame;
                    a.frame = 0;
    
                    // Continue back in forward animation
                    if schema.loops {
                        a.is_reversing = false;
                    }
                    // Return to idle
                    else {
                        Self::sprite_to_idle(s, a);
                        return;
                    }
                }
            }
        }
    }
}
impl<'a> System<'a> for AnimSpriteSys {
    type SystemData = (WriteStorage<'a, Sprite>,
                       WriteStorage<'a, Animation>,
                       Read<'a, DeltaTime>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut sprites, mut anims, dt) = data;
        let dt = dt.0;

        for (mut s, mut a) in (&mut sprites, &mut anims).join() {
            // Advance the Animation data
            Self::advance_animation(&mut s, &mut a, dt); 

            // Apply the animation changes, if it exists
            match &a.schema {
                Some(schema) => s.id = schema.root + (a.frame as u16),
                None => s.id = s.schema.root,
            };
        }
    }
}


#[derive(Default)]
pub struct SpriteRenderSys {
    renderer: SpriteRenderer,
}
impl<'a> System<'a> for SpriteRenderSys {
    type SystemData = (ReadStorage<'a, Sprite>,
                       ReadStorage<'a, Transform>,
                       ReadStorage<'a, Color>);

    fn run(&mut self, data: Self::SystemData) {
        let (sprites, xforms, colors) = data;
        
        // Build the RenderSprite Vec from the components
        let sprites: Vec<RenderSprite> = 
            (&sprites, &xforms, &colors).join()
                .map(|data| data.into())
                .collect();
        self.renderer.render(&sprites);
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.renderer = SpriteRenderer::new();
        self.renderer.init(include_bytes!("../../../assets/textures/atlas.png")).unwrap();
    }
}
