use crate::sprite::Sprite;

use sdl2::render::Texture;

const PLAYER_W: u32 = 26;
const PLAYER_H: u32 = 36;
const PLAYER_SPEED: i32 = 5;

// handles player, received input
//#[derive(Debug)]
pub struct Player<'a> {
    // todo: make not pub
    pub spr: Sprite<'a>
}

impl <'a> Player<'a> {
    pub fn new(spritesheet: Texture<'a>) -> Player {
        Self {
            spr: Sprite::new(PLAYER_W, PLAYER_H, PLAYER_SPEED, spritesheet)
        }
    }
}