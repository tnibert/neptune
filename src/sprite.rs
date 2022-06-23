// handles renderable character
//#[derive(Debug)]
pub struct Sprite {
    pub position: [f64; 4],
    //pub area: Rect,
    //pub speed: i32,
}

impl Sprite {
    pub fn new(/*width: u32, height: u32, speed: i32*/) -> Sprite {
        Self {
            position: [0.0, 0.0, 100.0, 100.0],
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
    }
}