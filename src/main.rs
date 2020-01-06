extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics,
    window: GlutinWindow,

    event_settings: EventSettings,
}

impl App {
    fn new(opengl: OpenGL) -> App {
        let window_settings = WindowSettings::new("opensi", [640, 480])
            .graphics_api(opengl)
            .exit_on_esc(true);
        let window: GlutinWindow = window_settings.build().unwrap();
        let event_settings = EventSettings::new();
        let gl = GlGraphics::new(opengl);

        App { gl, window, event_settings }
    }

    fn run(&mut self) {
        let mut events = Events::new(self.event_settings);

        while let Some(event) = events.next(&mut self.window) {
            if let Some(_args) = event.update_args() {
                // update
            }

            if let Some(_args) = event.render_args() {
                // render
            }
        }
    }
}

fn main() {
    let mut app = App::new(OpenGL::V2_1);
    app.run();
}
