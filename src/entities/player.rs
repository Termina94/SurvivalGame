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

use super::entity::{Collidable, Controllable, Entity, EntityState};

pub struct Player {
    state: EntityState,
}

impl Player {
    pub fn new(pos: Point) -> Self {
        Self {
            state: EntityState {
                id: Uuid::new_v4(),
                pos: pos,
                bounds: Dimensions::new(26, 32),
                speed: 200.0,
                y_velocity: 0.0,
                x_velocity: 0.0,
                sprite_asset_no: Sprites::PLAYER,
                colliding_entities: Vec::new(),
                hp: 500.0,
                damage: 0.0,
            },
        }
    }
}

impl Collidable for Player {}

impl Entity for Player {
    fn get_state(&self) -> &EntityState {
        &self.state
    }
    fn get_settings_mut(&mut self) -> &mut EntityState {
        &mut self.state
    }

    fn set_pos(&mut self, pos: Point) {
        self.state.pos = pos
    }

    fn update(&mut self, args: &UpdateArgs, state: &mut LevelState) {
        // If we are moving diagonal
        let speed = match self.state.x_velocity != 0.0 && self.state.y_velocity != 0.0 {
            // 0.707 is 45deg * π/180 as are always traveling at 45°
            true => self.state.speed * 0.707,
            false => self.state.speed,
        };

        let damage: f64 = self
            .state
            .colliding_entities
            .iter()
            .map(|enemy| enemy.borrow().get_state().damage)
            .sum();

        self.damage(&damage);

        println!("{}", self.state.hp);

        self.state.pos.x += self.state.x_velocity * speed * args.dt;
        self.state.pos.y += self.state.y_velocity * speed * args.dt;

        state.player_position = Point::from(self.state.pos.x, self.state.pos.y);
        self.state.colliding_entities.clear();
    }

    fn render(&mut self, args: &RenderArgs, state: &mut LevelState, gl: &mut GlGraphics) {
        let Point { x, y } = self.state.pos;
        let Dimensions { width, height } = self.state.bounds;

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(-width / 2.0, -height / 2.0);
            let box_transform = c
                .transform
                .trans(self.state.pos.x, self.state.pos.y)
                .trans(-(width / 2.0), -(height / 2.0));

            let color = match self.state.colliding_entities.len() {
                0 => BLUE,
                _ => RED,
            };

            rectangle(color, [x, y, width, height], transform, gl);

            if let Some(sprite) = state.get_texture(&self.state.sprite_asset_no) {
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
            Key::W => self.state.y_velocity = -1.0,
            Key::S => self.state.y_velocity = 1.0,
            Key::A => self.state.x_velocity = -1.0,
            Key::D => self.state.x_velocity = 1.0,
            _ => {}
        }
    }

    fn key_up(&mut self, key: &Key) {
        match key {
            Key::W if self.state.y_velocity < 0.0 => self.state.y_velocity = 0.0,
            Key::S if self.state.y_velocity > 0.0 => self.state.y_velocity = 0.0,
            Key::A if self.state.x_velocity < 0.0 => self.state.x_velocity = 0.0,
            Key::D if self.state.x_velocity > 0.0 => self.state.x_velocity = 0.0,
            _ => {}
        }
    }
}
