use super::Engine;
use glfw::{WindowEvent, Action};

// Aliases
pub type Key = glfw::Key;
pub type MouseButton = glfw::MouseButton;
pub type Modifiers = glfw::Modifiers;

impl<'a> Engine<'a> {
    pub fn handle_events(&mut self){
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events){
            match event {
                // Input events
                WindowEvent::Key(glfw::Key::Escape, _, Action::Press, _) => { 
                    self.window.set_should_close(true); 
                },
                WindowEvent::Key(key, _, Action::Press, modifiers) => {
                    self.game.key_down(key, modifiers);
                },
                WindowEvent::Key(key, _, Action::Release, modifiers) => {
                    self.game.key_up(key, modifiers);
                },
                WindowEvent::MouseButton(button, Action::Press, modifiers) => {
                    self.game.mouse_btn_down(button, modifiers);
                },
                WindowEvent::MouseButton(button, Action::Release, modifiers) => {
                    self.game.mouse_btn_up(button, modifiers);
                },

                // Default no op
                _ => {}
            }
        }
    }    
}
