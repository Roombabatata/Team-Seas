extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

use glam;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    fish: Fish,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLUE, gl);
        });
        self.fish.render(&mut self.gl, args);
    }

    fn update(&mut self, _args: &UpdateArgs) {
    }
}

struct Fish {
    pos: glam::Vec2,
}

impl Fish {
    fn render(&self, gl: &mut GlGraphics, _args: &RenderArgs) {

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = graphics::rectangle::square(self.pos.x as f64, self.pos.y as f64, 20_f64);
        gl.draw(_args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(RED, square, transform, gl);
        });
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Team seas", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),    
        fish : Fish {pos: glam::Vec2::new(10.0, 10.0)},
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}

