use std::{cell::RefCell, collections::HashMap, rc::Rc};

use opengl_graphics::{GlGraphics, Texture, TextureSettings};
use piston::{Button, Event, PressEvent, ReleaseEvent};
use rand::Rng;

use crate::{
    entities::{
        enemy::Enemy,
        entities::Entities,
        entity::{Collidable, Controllable, Entity},
        player::Player,
    },
    tools::{
        assets::{get_sprite_path, Sprites},
        helpers::Point,
    },
};

pub struct LevelState {
    pub debug: bool,
    pub player_position: Point,
    pub entity_textures: HashMap<Sprites, Option<Texture>>,
}

pub struct Level {
    pub state: LevelState,
    pub entities: Entities,
    pub required_assets: Vec<Sprites>,
}

impl LevelState {
    pub fn get_texture(&self, num: &Sprites) -> Option<&Texture> {
        self.entity_textures.get(num).unwrap_or(&None).as_ref()
    }
}

impl Level {
    pub fn new() -> Self {
        Self {
            state: LevelState {
                debug: false,
                player_position: Point::from(0, 0),
                entity_textures: HashMap::new(),
            },
            entities: Entities {
                player: Player::new(Point::from(0, 0)),
                entities: Vec::new(),
            },
            required_assets: vec![Sprites::PLAYER, Sprites::SKELETON],
        }
    }

    pub fn set_player_pos<T: Into<f64>>(&mut self, x: T, y: T) -> &mut Self {
        self.entities.player.set_pos(Point::from(x, y));

        self
    }

    pub fn spawn_enemies(&mut self) -> &mut Self {
        let mut rng = rand::thread_rng();

        self.entities.entities = (0..100)
            .collect::<Vec<u32>>()
            .iter()
            .map(|_| {
                let enemy: Rc<RefCell<dyn Collidable>> = Rc::new(RefCell::new(Enemy::new(
                    Point::from(rng.gen_range(0..1920), rng.gen_range(0..1080)),
                )));

                enemy
            })
            .collect();

        self
    }

    pub fn load_textures(&mut self) -> &mut Self {
        self.state.entity_textures = self
            .required_assets
            .iter()
            .map(|name| match get_sprite_path(name) {
                Some(path) => match Texture::from_path(path, &TextureSettings::new()) {
                    Ok(texture) => (*name, Some(texture)),
                    Err(_) => (*name, None),
                },
                None => (*name, None),
            })
            .collect();
        self
    }

    pub fn update(&mut self, args: &piston::UpdateArgs) {
        self.entities.update(args, &mut self.state);
    }

    pub fn render(&mut self, args: &piston::RenderArgs, gl: &mut GlGraphics) {
        self.entities.render(args, &mut self.state, gl);
    }
}

impl Controllable for Level {
    fn handle_input(&mut self, e: &Event) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            self.key_down(&key);
        }
        if let Some(Button::Keyboard(key)) = e.release_args() {
            self.key_up(&key);
        }
        self.entities.handle_input(e);
    }

    fn key_up(&mut self, key: &piston::Key) {
        match key {
            piston::Key::Return => self.state.debug = !self.state.debug,
            _ => {}
        }
    }
}
