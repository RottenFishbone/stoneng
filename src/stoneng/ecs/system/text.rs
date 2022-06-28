use specs::{ReadStorage, WriteStorage, System, Join, Read, SystemData};
use specs::prelude::*;
use crate::{
    ecs::component::{Color, Position, Text},
    ecs::resource::WindowSize,
    renderer::text::*,
};


#[derive(Default)]
pub struct TextRenderSys {
    renderer: TextRenderer,
}
impl<'a> System<'a> for TextRenderSys {
    type SystemData = (ReadStorage<'a, Text>,
                       ReadStorage<'a, Position>,
                       ReadStorage<'a, Color>,
                       Read<'a, WindowSize>);

    fn run(&mut self, data: Self::SystemData) {
        let (texts, pos, colors, window) = data;
        let window = (window.0, window.1);
        let texts: Vec<RenderString> = 
            (&texts, &pos, &colors).join()
                .map(|data| data.into())
                .collect();
        self.renderer.render(&texts, window, (0.0, 0.0, 0.0));
    }

    fn setup(&mut self, world: &mut World){ 
        Self::SystemData::setup(world);

        self.renderer = TextRenderer::new();
        self.renderer.init(
            include_bytes!("../../../../assets/textures/fonts/dogica.png"),
            8,
        ).unwrap();
    }
}
