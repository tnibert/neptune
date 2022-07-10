use crate::renderable::Renderable;
use crate::tile::{Tile, TILE_SIZE};

fn assemble_tiles(width: usize,
                  height: usize,
                  tile_map: Vec<u8>,
                  available_tiles: Vec<Tile>) -> im::RgbaImage {
    let mut img = im::RgbaImage::new((TILE_SIZE*width) as u32, (TILE_SIZE*height) as u32);
    for x in 0..width {
        for y in 0..height {
            println!("{} {}", x, y);
            let cur_tilemap_index: usize = x * y;
            println!("{}", cur_tilemap_index);
            im::imageops::overlay(&mut img,
                                  available_tiles[tile_map[cur_tilemap_index] as usize].render(),
                                  (x * TILE_SIZE) as i64,
                                  (y * TILE_SIZE) as i64);
        }
    }
    //im::imageops::overlay(&mut img, &on_top, 128, 128);
    return img;
}

pub struct TileArea {
    /*width: usize,
    height: usize,
    map: Vec<u8>,
    available_tiles: Vec<Tile>,*/
    image: im::RgbaImage
}

impl TileArea {
    pub fn new(width: usize) -> Self {
        let available_tiles = vec![Tile::new(), Tile::new()];

        let tile_map = vec![0, 0, 0,
                            0, 0, 0,
                            1, 0, 1];

        if tile_map.len() % width != 0 {
            panic!("The map is not rectangular!!");
        }
        let height = tile_map.len()/width;

        TileArea {
            /*width: width,
            height: height,
            map: tile_map,
            available_tiles: available_tiles,*/
            image: assemble_tiles(width, height, tile_map, available_tiles)
        }
    }
}

impl Renderable for TileArea {
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