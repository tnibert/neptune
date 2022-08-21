use crate::player::Player;
use crate::input::Input;
use crate::gameobject::GameObject;
use crate::tilemap::TileMap;
use crate::collision::Rect;

use crate::im::Pixel;

pub const SCREEN_WIDTH: u32 = 640;
pub const SCREEN_HEIGHT: u32 = 480;

pub struct Game {
    pub input: Input,
    gameobjects: Vec<Box<dyn GameObject>>
}

impl Game {
    pub fn new() -> Self {
        let player = Box::new(Player::new());

        let mytilemap = Box::new(TileMap::new(2));

        // setup subscriptions
        let mut input = Input::new();
        input.subscribe(player.observer.clone(), vec!["up", "down", "left", "right"]);

        Game {
            input: input,
            gameobjects: vec![mytilemap, player]
        }
    }
}

impl GameObject for Game {
    // create the screen image
    fn render(&self) -> Option<im::RgbaImage> {
        let mut screen_img = im::RgbaImage::new(SCREEN_WIDTH, SCREEN_HEIGHT);

        // clear screen
        // todo: find a more efficient call to do this
        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                screen_img.put_pixel(x, y, im::Rgb([255, 255, 255]).to_rgba());
            }
        }

        for g in &self.gameobjects {
            if let Some(img) = g.render() {
                if let Some(pos) = g.position() {
                    im::imageops::overlay(&mut screen_img, &img, pos.x as i64, pos.y as i64);
                } else {
                    continue;
                }
            } else {
                continue;
            }
        }

        return Some(screen_img);
    }

    fn position(&self) -> Option<Rect> {
        return Some(Rect{x: 0.0, y: 0.0, w: SCREEN_WIDTH as f64, h: SCREEN_HEIGHT as f64});
    }

    fn update(&mut self) {
        for u in self.gameobjects.iter_mut() {
            u.update();
        }
    }
}