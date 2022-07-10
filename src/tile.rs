use crate::im::Pixel;
use crate::renderable::Renderable;

pub const TILE_SIZE: usize = 32;       // all tiles are square

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Tile {
    //color: [f32; 4],
    size: usize,
    image: im::RgbaImage
}

impl Tile {
    pub fn new() -> Self {
        let mut img = im::RgbaImage::new(TILE_SIZE as u32, TILE_SIZE as u32);

        for x in 15..=17 {
            for y in 8..24 {
                img.put_pixel(x, y, im::Rgb([255, 0, 0]).to_rgba());
                img.put_pixel(y, x, im::Rgb([255, 0, 0]).to_rgba());
            }
        }

        Tile {
            //color: [1.0, 1.0, 1.0, 1.0],
            size: TILE_SIZE,
            image: img
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

