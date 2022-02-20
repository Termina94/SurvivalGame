use std::collections::HashMap;

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
use uuid::Uuid;

use super::entity::{Collidable, Entity};

pub struct Enemy {
    pub id: Uuid,
    pub pos: Point,
    pub bounds: Dimensions,
    pub y_velocity: f64,
    pub x_velocity: f64,
    pub speed: f64,
    pub sprite_asset_no: Sprites,
    pub colliding_entities: HashMap<Uuid, String>,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            pos: Point { x: 0.0, y: 0.0 },
            bounds: Dimensions::new(22, 33),
            speed: 50.0,
            y_velocity: 0.0,
            x_velocity: 0.0,
            sprite_asset_no: Sprites::SKELETON,
            colliding_entities: HashMap::new(),
        }
    }
}

impl Collidable for Enemy {
    fn collides(&mut self, entity: &Box<dyn Collidable>) {
        self.colliding_entities
            .insert(entity.get_id(), String::from("test"));
    }
}

impl Entity for Enemy {
    fn get_id(&self) -> Uuid {
        self.id
    }
    fn get_position(&self) -> Point {
        self.pos
    }
    fn get_bounds(&self) -> Dimensions {
        self.bounds
    }

    fn set_pos(&mut self, pos: Point) {
        self.pos = pos
    }

    fn update(&mut self, args: &UpdateArgs, state: &mut LevelState) {
        self.x_velocity = match self.pos.x - state.player_position.x {
            x if x > 0.0 => -1.0,
            x if x < 0.0 => 1.0,
            _ => 0.0,
        };
        self.y_velocity = match self.pos.y - state.player_position.y {
            y if y > 0.0 => -1.0,
            y if y < 0.0 => 1.0,
            _ => 0.0,
        };

        self.pos.y += self.y_velocity * (self.speed * args.dt);
        self.pos.x += self.x_velocity * (self.speed * args.dt);
    }

    fn render(self: &mut Enemy, args: &RenderArgs, state: &mut LevelState, gl: &mut GlGraphics) {
        let Dimensions { width, height } = self.bounds;

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(0.0, 0.0);
            let box_transform = c
                .transform
                .trans(self.pos.x, self.pos.y)
                .trans(-(width / 2.0), -(height / 2.0));

            if let Some(sprite) = state.get_texture(&self.sprite_asset_no) {
                image(sprite, box_transform, gl);
            }

            if state.debug {
                self.draw_hitboxes(transform, gl);
            }
        });
    }
}
