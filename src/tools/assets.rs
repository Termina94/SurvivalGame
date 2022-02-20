use std::collections::HashMap;

lazy_static! {
    static ref ASSETS: HashMap<Sprites, &'static str> = [
        (Sprites::PLAYER, "assets/sprites/player.png"),
        (Sprites::SKELETON, "assets/sprites/skel.gif"),
        (Sprites::Ghost, "test"),
        (Sprites::Goblin, "test"),
        (Sprites::Imp, "test"),
    ]
    .into_iter()
    .collect();
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum Sprites {
    PLAYER = 0,
    SKELETON,
    Ghost,
    Goblin,
    Imp,
}

pub fn get_sprite_path(sprite: &Sprites) -> Option<&'static str> {
    match ASSETS.get(&sprite) {
        Some(path) => Some(*path),
        None => None,
    }
}
