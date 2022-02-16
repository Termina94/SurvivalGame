extern crate getset;
extern crate glutin_window;
extern crate graphics;
extern crate lazy_static;
extern crate opengl_graphics;
extern crate piston;

use std::path::Path;

use entities::enemy::Enemy;
use entities::entities::Entities;
use entities::entity::Collidable;
use entities::player::Player;
use glutin_window::GlutinWindow as Window;
use graphics::clear;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use rand::Rng;
use tools::helpers::Point;

mod entities;
mod tools;

pub struct Level {
    pub debug: bool,
    pub player_position: (f64, f64),
    pub enemy_texture: Texture,
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut rng = rand::thread_rng();

    let mut window: Window = WindowSettings::new("Vampire Ripoff", [1920, 1080])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut level = Level {
        debug: true,
        player_position: (0.0, 0.0),
        enemy_texture: Texture::from_path(
            Path::new("assets/sprites/skel.gif"),
            &TextureSettings::new(),
        )
        .unwrap(),
    };

    let mut entities = Entities {
        player: Player {
            pos: Point::new(1920 / 2, 1080 / 2),
            ..Default::default()
        },
        entities: (0..10)
            .collect::<Vec<u32>>()
            .iter()
            .map(|_| {
                let enemy: Box<dyn Collidable> = Box::new(Enemy {
                    pos: Point::new(rng.gen_range(0..1920), rng.gen_range(0..1080)),
                    ..Default::default()
                });

                enemy
            })
            .collect(),
    };

    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        entities.handle_input(&e);

        if let Some(args) = e.update_args() {
            entities.update(&args, &mut level);
        }

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |_, gl| {
                clear([0.0, 0.0, 0.0, 0.0], gl);
            });

            entities.render(&args, &mut level, &mut gl);
        }
    }
}
