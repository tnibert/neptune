use crate::renderable::Render;
use crate::tile::{Tile, TILE_SIZE};
use crate::im::Pixel;

// size of tile register - e.g. up to 256 unique tiles
type tile_reg = u8;

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
                    //img.put_pixel(x, y, im::Rgb([255, 0, 0]).to_rgba());
                    img.put_pixel(y, x, im::Rgb([0, 0, 0]).to_rgba());
                }
            }
            img
        })
    ]
}

fn assemble_tiles_to_image(width: usize,
                           height: usize,
                           tile_map: Vec<TileMeta>,
                           available_tiles: Vec<Tile>) -> im::RgbaImage {
    let mut img = im::RgbaImage::new((TILE_SIZE*width) as u32, (TILE_SIZE*height) as u32);
    let mut cur_tilemap_index: usize = 0;
    for y in 0..height {
        for x in 0..width {
            println!("{}: {}", cur_tilemap_index, tile_map[cur_tilemap_index].tile_register_index);
            im::imageops::overlay(&mut img,
                                  available_tiles[tile_map[cur_tilemap_index].tile_register_index as usize].render(),
                                  (x * TILE_SIZE) as i64,
                                  (y * TILE_SIZE) as i64);
            cur_tilemap_index += 1;
        }
    }
    return img;
}

// Tile + metadata
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct TileMeta {
    tile_register_index: tile_reg,
    walkoverable: bool
}

// A grouping of tiles into one image
pub struct TileArea {
    /*width: usize,
    height: usize,
    map: Vec<u8>,
    available_tiles: Vec<Tile>,*/
    image: im::RgbaImage
}

impl TileArea {
    pub fn new(width: usize) -> Self {
        let c = TileMeta {
            tile_register_index: 1,
            walkoverable: false
        };
        let r = TileMeta {
            tile_register_index: 0,
            walkoverable: true
        };

        let tile_map = vec![c, r, r,
                            r, r, r,
                            c, r, c];

        if tile_map.len() % width != 0 {
            panic!("The map is not rectangular!!");
        }
        let height = tile_map.len() / width;

        TileArea {
            /*width: width,
            height: height,
            map: tile_map,
            available_tiles: available_tiles,*/
            image: assemble_tiles_to_image(width, height, tile_map, create_tile_set())
        }
    }
}

impl Render for TileArea {
    fn render(&self) -> &im::RgbaImage {
        return &self.image;
    }

    fn position(&self) -> (f64, f64) {
        return (300.0, 300.0);
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let test = TileArea::new(3);
    }
}