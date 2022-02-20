extern crate getset;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

#[macro_use]
extern crate lazy_static;

use entities::entity::Controllable;
use glutin_window::GlutinWindow as Window;
use graphics::clear;
use levels::level::Level;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

mod entities;
mod levels;
mod tools;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Vampire Ripoff", [1920, 1080])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut level = Level::new();
    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());

    level
        .set_player_pos(1920 / 2, 1080 / 2)
        .load_textures()
        .spawn_enemies();

    while let Some(e) = events.next(&mut window) {
        level.handle_input(&e);

        if let Some(args) = e.update_args() {
            level.update(&args);
        }

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |_, gl| {
                clear([0.0, 0.0, 0.0, 0.0], gl);
            });

            level.render(&args, &mut gl);
        }
    }
}
