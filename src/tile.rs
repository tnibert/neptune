use crate::im::Pixel;
use crate::renderable::Renderable;

const TILE_SIZE: u32 = 32;       // all tiles are square

pub struct Tile {
    //color: [f32; 4],
    size: u32,
    image: im::RgbaImage
}

impl Tile {
    pub fn new() -> Self {
        let mut img = im::RgbaImage::new(TILE_SIZE, TILE_SIZE);

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

