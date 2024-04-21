use crate::background::Background;
use crate::npc::NPC;
use crate::observer::Listener;
use crate::player::Player;
use crate::input::Input;
use crate::gameobject::GameObject;
use crate::collision::Rect;
use crate::sprite::Direction;
use std::rc::Rc;
//use std::time::Instant;

pub const SCREEN_WIDTH: u32 = 640;
pub const SCREEN_HEIGHT: u32 = 480;
pub const FRAME_RATE: u32 = 45;

pub struct Game {
    pub input: Input,
    gameobjects: Vec<Box<dyn GameObject>>,
    observer: Rc<Listener>
}

impl Game {
    pub fn new() -> Self {
        let game_observer = Rc::new(Listener::new());

        let player = Box::new(Player::new());
        let npc = Box::new(NPC::new(Direction::Right));

        //let mytilemap = Box::new(TileMap::new(2));
        let mut bg = Box::new(Background::new("map.jpg"));
        bg.signals_out.subscribe("visibility_change".to_string(), game_observer.clone());

        let mut input = Input::new();
        input.subscribe(player.observer.clone(), vec!["up", "down", "left", "right"]);
        input.subscribe(bg.observer.clone(), vec!["up", "down", "left", "right"]);
        input.subscribe(npc.observer.clone(), vec!["up", "down", "left", "right"]);

        Game {
            input: input,
            gameobjects: vec![bg, player, npc],
            observer: game_observer
        }
    }
}

impl GameObject for Game {
    // create the screen image
    fn render(&self) -> Option<im::RgbaImage> {
        let mut screen_img = im::RgbaImage::new(SCREEN_WIDTH, SCREEN_HEIGHT);

        for g in &self.gameobjects {
            
            if let Some(img) = g.render() {
                if let Some(pos) = g.position() {
                    //let start = Instant::now();

                    // todo: collision::convert_world_coord_to_screen_coord()
                    im::imageops::overlay(&mut screen_img, &img, pos.x as i64, pos.y as i64);

                    //let duration = start.elapsed();
                    //println!("overlay: {:?}", duration);
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

        for e in self.observer.poll_evt() {
            match e.name.as_str() {
                "visibility_change" => {
                    // todo: track bounding rect
                    println!("visibility");
                },
                _ => ()
            }
        }
    }
}