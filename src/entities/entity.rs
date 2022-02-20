use crate::{
    levels::level::LevelState,
    tools::helpers::{draw_rect, Dimensions, Point},
};
use graphics::{
    color::{GREEN, RED},
    math::Matrix2d,
    rectangle,
};
use opengl_graphics::GlGraphics;
use piston::{Button, Event, Key, PressEvent, ReleaseEvent, RenderArgs, UpdateArgs};
use uuid::Uuid;

pub trait Entity {
    fn get_id(&self) -> Uuid;
    fn get_position(&self) -> Point;
    fn get_bounds(&self) -> Dimensions;
    fn get_sprite(&self) -> Option<String> {
        None
    }

    fn set_pos(&mut self, pos: Point);

    fn update(&mut self, args: &UpdateArgs, state: &mut LevelState);
    fn render(&mut self, args: &RenderArgs, state: &mut LevelState, gl: &mut GlGraphics);
}

pub trait Controllable {
    fn handle_input(&mut self, e: &Event) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            self.key_down(&key);
        }
        if let Some(Button::Keyboard(key)) = e.release_args() {
            self.key_up(&key);
        }
    }
    fn key_down(&mut self, _: &Key) {}
    fn key_up(&mut self, _: &Key) {}
}

pub trait Collidable: Entity {
    fn collides(&mut self, entity: &Box<dyn Collidable>);

    fn does_collide(&self, entity: &Box<dyn Collidable>) -> bool {
        let [x1, y1, width1, height1] = self.get_hitbox();
        let [x2, y2, width2, height2] = entity.get_hitbox();

        x1 < x2 + width2 && x1 + width1 > x2 && y1 < y2 + height2 && height1 + y1 > y2
    }

    fn get_hitbox(&self) -> [f64; 4] {
        let Point { x, y } = self.get_position();
        let Dimensions { width, height } = self.get_bounds();

        [x - (width / 2.0), y - (height / 2.0), width, height]
    }

    fn draw_hitboxes(&mut self, transform: Matrix2d, gl: &mut GlGraphics) {
        let Point { x, y } = self.get_position();

        // center dot
        rectangle(RED, [x - 2.0, y - 2.0, 4.0, 4.0], transform, gl);
        // Collision box
        draw_rect(GREEN, self.get_hitbox(), transform, gl);
    }
}
