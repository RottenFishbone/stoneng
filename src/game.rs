#![allow(unused_variables)]

use crate::engine::{Key, Modifiers, MouseButton};

pub struct RustyLantern {

}

impl crate::engine::GameCore for RustyLantern {
    fn tick(&mut self) {
    }

    fn key_up(&mut self, key: Key, modifiers: Modifiers) {
    }

    fn key_down(&mut self, key: Key, modifiers: Modifiers) {
    }

    fn click_up(&mut self, button: MouseButton, modifiers: Modifiers) {
    }

    fn click_down(&mut self, button: MouseButton, modifiers: Modifiers) {
    }
    
}

