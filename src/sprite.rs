use crate::graphics::*;

// todo: pass these in as parameters, or determine from file
const SPRITE_FRAME_W: u32 = 96/3;
const SPRITE_FRAME_H: u32 = 144/4;
const SS_DOWN: u32 = 4;
const SS_ACROSS: u32 = 3;
const MAX_FRAME: usize = (SS_DOWN * SS_ACROSS - 1) as usize;

fn load_spritesheet(img: &im::RgbaImage) -> Vec<im::RgbaImage> {
    let mut v: Vec<im::RgbaImage> = Vec::new();
    for y in 0..SS_DOWN {
        for x in 0..SS_ACROSS {
            println!("{}, {}", x, y);
            let frame_x_start = x * SPRITE_FRAME_W;
            let frame_y_start = y * SPRITE_FRAME_H;
            v.push(im::imageops::crop_imm(img, frame_x_start, frame_y_start, SPRITE_FRAME_W, SPRITE_FRAME_H).to_image());
        }
    }
    return v;
}

// handles renderable character
//#[derive(Debug)]
pub struct Sprite {
    pub position: [f64; 4],
    frame: usize,
    //pub spritesheet: im::RgbaImage,
    frames: Vec<im::RgbaImage>
    //pub area: Rect,
    //pub speed: i32,
}

impl Sprite {
    pub fn new() -> Sprite {
        //let ss = load_image_asset_buffer("reaper.png");
        Self {
            position: [0.0, 0.0, 100.0, 100.0],
            frame: 0,
            //spritesheet: ss,
            // todo: pass in filename
            frames: load_spritesheet(&load_image_asset_buffer("reaper.png"))
            // src position in the spritesheet
            //area: Rect::new(0, 0, width, height),
            //speed: speed,
            //spritesheet: load_sprite_img(&window)
        }
    }

    pub fn movespr(&mut self, xdiff: f64, ydiff: f64) {
        // todo: bounds checking
        self.position[0] += xdiff;
        self.position[1] += ydiff;
        println!("{:?}", self.position);

        // todo: update so frame chosen based on direction travelling
        if self.frame < MAX_FRAME {
            self.frame += 1;
        } else {
            self.frame = 0;
        }
    }

    pub fn current_frame(&self) -> &im::RgbaImage {
        return &self.frames[self.frame];
    }
}