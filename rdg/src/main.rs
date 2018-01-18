extern crate ai_behavior;
extern crate opengl_graphics;
extern crate piston_window;
extern crate rand;
extern crate sprite;


/*
mod settings;
mod platforms;
*/
pub mod game;
pub mod entities;

use piston_window::{OpenGL, PistonWindow, Size, WindowSettings};
use opengl_graphics::GlGraphics;

fn main() {
    let name = "I'm bad at catch";
    let opengl = OpenGL::V3_2;
    let window_size = Size {
        width: 1024,
        height: 720,
    };

    let mut window: PistonWindow = WindowSettings::new(
        name,
        [window_size.width, window_size.height],
        ).opengl(opengl)
        .samples(8)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|error| panic!("{}", error));
    let mut gl = GlGraphics::new(opengl);
    let mut g = game::Game::new(window_size);
    g.run(&mut window, &mut gl);
}
