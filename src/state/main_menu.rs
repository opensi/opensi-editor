use crate::state::{State,WithoutData};

pub struct MainMenu;

impl State for MainMenu {
    fn on_update(&mut self, delta: f32) {
        // todo
    }

    fn on_render(&self) {
        // todo
    }
}

impl WithoutData for MainMenu {
    fn new() -> Self {
        MainMenu
    }
}
