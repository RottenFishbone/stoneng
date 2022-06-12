pub mod sprite;
pub mod light;
pub mod text;

use specs::prelude::*;

#[derive(Default)]
pub struct RenderSys;
impl<'a> System<'a> for RenderSys {
    type SystemData = ();

    fn run(&mut self, data: Self::SystemData) {
        unsafe {
            gl::ClearColor(0.2, 0.2, 0.25, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
}
