pub struct Tile {
    color: [f32; 4],
    //pub position: [f64; 4],
}

impl Tile {
    pub fn new() -> Self {
        Tile {
            color: [1.0, 1.0, 1.0, 1.0],
            //position: [0.0, 0.0, 100.0, 100.0]
        }
    }

    pub fn update(&mut self, dt: f64, _size: (f64, f64)) {
        self.color[0] = Self::update_color(dt as f32, self.color[0], 0.001);
        self.color[1] = Self::update_color(dt as f32, self.color[1], 0.002);
        self.color[2] = Self::update_color(dt as f32, self.color[2], 0.003);
    }

    fn update_color(dt: f32, color: f32, change: f32)->f32 {
        if color <= 0.0 {
            1.0
        } else {
            color - change * dt * 120.0
        }
    }
}