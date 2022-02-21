use crate::{
    levels::level::LevelState,
    tools::{
        assets::Sprites,
        helpers::{Dimensions, Point},
    },
};
use graphics::{image, Transformed};
use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};
use std::collections::HashMap;
use uuid::Uuid;

use super::entity::{Collidable, Entity, EntityState};

pub struct Enemy {
    state: EntityState,
}

impl Enemy {
    pub fn new(pos: Point) -> Self {
        Self {
            state: EntityState {
                id: Uuid::new_v4(),
                pos: pos,
                bounds: Dimensions::new(22, 33),
                speed: 50.0,
                y_velocity: 0.0,
                x_velocity: 0.0,
                sprite_asset_no: Sprites::SKELETON,
                colliding_entities: HashMap::new(),
            },
        }
    }
}

impl Collidable for Enemy {}

impl Entity for Enemy {
    fn get_state(&self) -> &EntityState {
        &self.state
    }
    fn get_settings_mut(&mut self) -> &mut EntityState {
        &mut self.state
    }

    fn update(&mut self, args: &UpdateArgs, state: &mut LevelState) {
        self.state.x_velocity = match self.state.pos.x - state.player_position.x {
            x if x > 0.0 => -1.0,
            x if x < 0.0 => 1.0,
            _ => 0.0,
        };
        self.state.y_velocity = match self.state.pos.y - state.player_position.y {
            y if y != 0.0 => -1.0,
            _ => 0.0,
        };

        let Point { x, y } = self.calculate_movement_vectors(&state.player_position);

        self.state.pos.y += self.state.y_velocity * (x * args.dt);
        self.state.pos.x += self.state.x_velocity * (y * args.dt);
    }

    fn render(self: &mut Enemy, args: &RenderArgs, state: &mut LevelState, gl: &mut GlGraphics) {
        let Dimensions { width, height } = self.state.bounds;

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(0.0, 0.0);
            let box_transform = c
                .transform
                .trans(self.state.pos.x, self.state.pos.y)
                .trans(-(width / 2.0), -(height / 2.0));

            if let Some(sprite) = state.get_texture(&self.state.sprite_asset_no) {
                image(sprite, box_transform, gl);
            }

            if state.debug {
                self.draw_hitboxes(transform, gl);
            }
        });
    }
}
