use crate::background::Background;
use crate::collision;
use crate::npc::NPC;
use crate::observer::Listener;
use crate::observer::NeptuneEvent;
use crate::player::Player;
use crate::input::Input;
use crate::gameobject::GameObject;
use crate::collision::Rect;
use crate::sprite::Direction;
use crate::talkbubble::TalkBubble;
use crate::viewport::Viewport;
use std::rc::Rc;
//use std::time::Instant;

pub const SCREEN_WIDTH: u32 = 640;
pub const SCREEN_HEIGHT: u32 = 480;
pub const FRAME_RATE: u32 = 45;

pub struct Game {
    pub input: Input,
    gameobjects: Vec<Box<dyn GameObject>>,
    observer: Rc<Listener>,
    visiblescene: Viewport
}

impl Game {
    pub fn new() -> Self {
        let game_observer = Rc::new(Listener::new());

        let npc = Box::new(NPC::new(Direction::Right, collision::new_point((SCREEN_WIDTH + 30) as i64, (SCREEN_HEIGHT + 30) as i64)));
        let bubble = Box::new(TalkBubble::new());

        //let mytilemap = Box::new(TileMap::new(2));
        let mut bg = Box::new(Background::new("map.jpg"));
        bg.signals_out.subscribe(NeptuneEvent::VisibilityChange(Viewport::new_default()), game_observer.clone());

        // todo: do we need the Box::new() above?
        let mut player = Box::new(Player::new(Rect{x: 0, y: 0, w: bg.width() as i64, h: bg.height() as i64}));
        let mut input = Input::new();
        input.subscribe(player.observer.clone(), vec![NeptuneEvent::Up, NeptuneEvent::Down, NeptuneEvent::Left, NeptuneEvent::Right]);
        player.subscribe(bg.observer.clone(), vec![NeptuneEvent::Up, NeptuneEvent::Down, NeptuneEvent::Left, NeptuneEvent::Right]);
        player.subscribe(npc.observer.clone(), vec![NeptuneEvent::Up, NeptuneEvent::Down, NeptuneEvent::Left, NeptuneEvent::Right]);

        bg.signals_out.subscribe(NeptuneEvent::VisibilityChange(Viewport::new_default()), player.observer.clone());

        let visible_scene = Viewport::new(bg.position().unwrap());

        Game {
            input: input,
            gameobjects: vec![bg, player, npc, bubble],
            observer: game_observer,
            visiblescene: visible_scene
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

                    let screen_pos = self.visiblescene.screen_coordinates(&pos);

                    im::imageops::overlay(&mut screen_img, &img, screen_pos.x as i64, screen_pos.y as i64);

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
        return Some(Rect{x: 0, y: 0, w: SCREEN_WIDTH as i64, h: SCREEN_HEIGHT as i64});
    }

    fn update(&mut self) {
        for u in self.gameobjects.iter_mut() {
            u.update();
        }

        for e in self.observer.poll_evt() {
            match e {
                NeptuneEvent::VisibilityChange(r) => {
                    self.visiblescene = r;
                },
                _ => ()
            }
        }
    }
}