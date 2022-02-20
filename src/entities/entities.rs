use super::{
    entity::{Collidable, Controllable, Entity},
    player::Player,
};
use crate::levels::level::LevelState;
use opengl_graphics::GlGraphics;
use piston::{Event, RenderArgs, UpdateArgs};

pub struct Entities {
    pub player: Player,
    pub entities: Vec<Box<dyn Collidable>>,
}

impl Entities {
    pub fn update(&mut self, args: &UpdateArgs, state: &mut LevelState) {
        self.player.update(args, state);

        for entity in self.entities.iter_mut() {
            if self.player.does_collide(entity) {
                self.player.collides(entity);
                // entity.collides(&self.player);
            }

            entity.update(args, state);
        }
    }

    pub fn render(&mut self, args: &RenderArgs, state: &mut LevelState, gl: &mut GlGraphics) {
        self.player.render(args, state, gl);

        for entity in self.entities.iter_mut() {
            entity.render(args, state, gl);
        }
    }
}

impl Entities {
    pub fn handle_input(&mut self, e: &Event) {
        self.player.handle_input(e);
    }
}
