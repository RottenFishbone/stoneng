
use glutin::event;

pub type KeyCode = event::VirtualKeyCode;
pub type ElementState = event::ElementState;
pub type MouseButton = event::MouseButton;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct KeyEvent {
    pub key: Option<KeyCode>,
    pub state: ElementState, 
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MouseBtnEvent {
    pub button: MouseButton,
    pub state: ElementState,
}

impl From<event::KeyboardInput> for KeyEvent {
    fn from(e: event::KeyboardInput) -> Self {
        KeyEvent { 
            key: e.virtual_keycode,
            state: e.state
        }
    }
}

