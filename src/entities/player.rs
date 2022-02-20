use std::collections::HashMap;

use graphics::{
    color::{BLUE, RED},
    image, rectangle, Transformed,
};
use opengl_graphics::GlGraphics;
use piston::{Key, RenderArgs, UpdateArgs};
use uuid::Uuid;

use crate::{
    levels::level::LevelState,
    tools::{
        assets::Sprites,
        helpers::{Dimensions, Point},
    },
};

use super::entity::{Collidable, Controllable, Entity};

pub struct Player {
    pub id: Uuid,
    pub pos: Point,
    pub bounds: Dimensions,
    pub y_velocity: f64,
    pub x_velocity: f64,
    pub speed: f64,
    pub sprite_asset_no: Sprites,
    pub colliding_entities: HashMap<Uuid, String>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            pos: Point { x: 0.0, y: 0.0 },
            bounds: Dimensions::new(26, 32),
            speed: 200.0,
            y_velocity: 0.0,
            x_velocity: 0.0,
            sprite_asset_no: Sprites::PLAYER,
            colliding_entities: HashMap::new(),
        }
    }
}

impl Collidable for Player {
    fn collides(&mut self, entity: &Box<dyn Collidable>) {
        self.colliding_entities
            .insert(entity.get_id(), String::from("test"));
    }
}

impl Entity for Player {
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
        self.pos.y += self.y_velocity * (self.speed * args.dt);
        self.pos.x += self.x_velocity * (self.speed * args.dt);

        state.player_position = Point::from(self.pos.x, self.pos.y);
    }

    fn render(&mut self, args: &RenderArgs, state: &mut LevelState, gl: &mut GlGraphics) {
        let Point { x, y } = self.pos;
        let Dimensions { width, height } = self.bounds;

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(-width / 2.0, -height / 2.0);
            let box_transform = c
                .transform
                .trans(self.pos.x, self.pos.y)
                .trans(-(width / 2.0), -(height / 2.0));

            let color = match self.colliding_entities.len() {
                0 => BLUE,
                _ => RED,
            };

            rectangle(color, [x, y, width, height], transform, gl);

            if let Some(sprite) = state.get_texture(&self.sprite_asset_no) {
                image(sprite, box_transform, gl);
            }

            if state.debug {
                self.draw_hitboxes(c.transform, gl);
            }
        });
    }
}

impl Controllable for Player {
    fn key_down(&mut self, key: &Key) {
        match key {
            Key::W => self.y_velocity = -1.0,
            Key::S => self.y_velocity = 1.0,
            Key::A => self.x_velocity = -1.0,
            Key::D => self.x_velocity = 1.0,
            _ => {}
        }
    }

    fn key_up(&mut self, key: &Key) {
        match key {
            Key::W if self.y_velocity < 0.0 => self.y_velocity = 0.0,
            Key::S if self.y_velocity > 0.0 => self.y_velocity = 0.0,
            Key::A if self.x_velocity < 0.0 => self.x_velocity = 0.0,
            Key::D if self.x_velocity > 0.0 => self.x_velocity = 0.0,
            _ => {}
        }
    }
}
