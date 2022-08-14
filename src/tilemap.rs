use crate::gameobject::GameObject;
use crate::tile::{Tile, TILE_SIZE};
use crate::collision::Rect;
use crate::im::Pixel;

pub struct TileMap {
    tiles: Vec<Tile>,
    width: usize            // width in terms of number of tiles
}

// a map of tiles
impl TileMap {
    pub fn new(width: usize) -> Self {
        // test data for tiles, todo: pass in
        let tiles = vec![
            // cross
            Tile::new(|| {
                let mut img = im::RgbaImage::new(TILE_SIZE as u32, TILE_SIZE as u32);

                for x in 15..=17 {
                    for y in 8..24 {
                        img.put_pixel(x, y, im::Rgb([255, 0, 0]).to_rgba());
                        img.put_pixel(y, x, im::Rgb([255, 0, 0]).to_rgba());
                    }
                }
                img
            }, true),
            // black square
            Tile::new(|| {
                let mut img = im::RgbaImage::new(TILE_SIZE as u32, TILE_SIZE as u32);

                for x in 0..TILE_SIZE as u32 {
                    for y in 0..TILE_SIZE as u32 {
                        img.put_pixel(x, y, im::Rgb([0, 0, 0]).to_rgba());
                    }
                }
                img
            }, false)
        ];

        if tiles.len() % width != 0 {
            panic!("The map is not rectangular!!");
        }
        
        TileMap {
            tiles: tiles,
            width: width
        }
    }

    pub fn query_permeability_at_location(&self) -> bool {
        true
    }

    fn get_width(&self) -> usize {
        return self.width;
    }

    fn get_height(&self) -> usize {
        return self.tiles.len() / self.width;
    }
}

impl GameObject for TileMap {
    fn render(&self) -> Option<im::RgbaImage> {
        let mut img = im::RgbaImage::new((TILE_SIZE*self.get_width()) as u32, (TILE_SIZE*self.get_height()) as u32);

        let mut step = 0;
        for t in &self.tiles {
            if let Some(tile_img) = t.render() {
                im::imageops::overlay(&mut img,
                    &tile_img,
                    ((step % self.get_width()) * TILE_SIZE) as i64,
                    ((step / self.get_width()) * TILE_SIZE) as i64);
            } else {
                panic!("tile did not render");
            }
            
            step += 1;
        }
        return Some(img);
    }

    fn position(&self) -> Option<Rect> {
        return Some(Rect{x: 50.0, y: 50.0,
            w: (self.get_width() * TILE_SIZE) as f64,
            h: (self.get_height() * TILE_SIZE) as f64});
    }

    fn update(&mut self) {}
}