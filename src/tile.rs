use crate::renderable::Renderable;

pub const TILE_SIZE: usize = 32;       // all tiles are square

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Tile {
    //color: [f32; 4],
    size: usize,
    image: im::RgbaImage
}

impl Tile {
    pub fn new<F: Fn() -> im::RgbaImage>(f: F) -> Self {
        Tile {
            //color: [1.0, 1.0, 1.0, 1.0],
            size: TILE_SIZE,
            image: f()
        }
    }
}

impl Renderable for Tile {
    fn render(&self) -> &im::RgbaImage {
        return &self.image;
    }

    fn position(&self) -> (f64, f64) {
        return (100.0, 100.0);
    }
}

