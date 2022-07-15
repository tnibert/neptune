use crate::renderable::Render;
use crate::tile::{Tile, TILE_SIZE};
use crate::collision::Rect;
use crate::im::Pixel;

// size of tile register - e.g. up to 256 unique tiles
type TileRegIndex = u8;

// todo: encapsulate this in some kind of factory, allow different tile sets to be created
fn create_tile_set() -> Vec<Tile> {
    vec![
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
        }),
        // black square
        Tile::new(|| {
            let mut img = im::RgbaImage::new(TILE_SIZE as u32, TILE_SIZE as u32);
    
            for x in 0..TILE_SIZE as u32 {
                for y in 0..TILE_SIZE as u32 {
                    img.put_pixel(y, x, im::Rgb([0, 0, 0]).to_rgba());
                }
            }
            img
        })
    ]
}

fn assemble_tiles_to_image(width: usize,
                           height: usize,
                           tile_map: &Vec<TileRegIndex>,
                           available_tiles: Vec<Tile>) -> im::RgbaImage {
    let mut img = im::RgbaImage::new((TILE_SIZE*width) as u32, (TILE_SIZE*height) as u32);
    let mut cur_tilemap_index: usize = 0;
    for y in 0..height {
        for x in 0..width {
            //println!("{}: {}", cur_tilemap_index, tile_map[cur_tilemap_index]);
            im::imageops::overlay(&mut img,
                                  available_tiles[tile_map[cur_tilemap_index] as usize].render(),
                                  (x * TILE_SIZE) as i64,
                                  (y * TILE_SIZE) as i64);
            cur_tilemap_index += 1;
        }
    }
    return img;
}

// this is temporary
pub fn create_tile_map() -> Vec<TileRegIndex> {
    vec![1, 0, 0,
         0, 0, 0,
         1, 0, 1]
}

// maybe move to collision.rs?
// ability to move through a TileArea
pub enum Permeability {
    Permeable,
    NonPermeable,
    FromChild
}

// A grouping of tiles into one image
pub struct TileArea {
    position: Rect,
    permeability: Permeability,
    map: Vec<TileRegIndex>,
    image: im::RgbaImage
}

impl TileArea {
    pub fn new(width: usize, tile_map: Vec<TileRegIndex>) -> Self {
        if tile_map.len() % width != 0 {
            panic!("The map is not rectangular!!");
        }
        let height = tile_map.len() / width;

        let img = assemble_tiles_to_image(width, height, &tile_map, create_tile_set());

        TileArea {
            position: Rect {
                x: 200.0,
                y: 200.0,
                w: width as f64,
                h: height as f64
            },
            permeability: Permeability::Permeable,
            map: tile_map,
            image: img
        }
    }
}

impl Render for TileArea {
    fn render(&self) -> &im::RgbaImage {
        return &self.image;
    }

    fn position(&self) -> (f64, f64) {
        return (self.position.x, self.position.y);
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let test = TileArea::new(3, create_tile_map());
    }
}