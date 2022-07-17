use crate::gameobject::GameObject;

pub const TILE_SIZE: usize = 32;       // all tiles are square

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Tile {
    //color: [f32; 4],
    size: usize,                      // size of one side of square
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

// perhaps a tile should not be renderable without a TileArea
impl GameObject for Tile {
    fn render(&self) -> Option<im::RgbaImage> {
        return Some(self.image.clone());
    }

    fn position(&self) -> Option<(f64, f64)> {
        return Some((100.0, 100.0));
    }

    fn update(&mut self) {}
}

