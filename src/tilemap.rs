use crate::gameobject::GameObject;
use crate::tile::{Tile, TILE_SIZE};
use crate::game::{SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::collision::Rect;
use crate::im::Pixel;

// todo: remove unnecessary casts to u32

pub struct TileMap {
    tiles: Vec<Tile>,
    width: usize            // width in terms of number of tiles
}

// a map of tiles
impl TileMap {
    pub fn new(width: usize) -> Self {
        // test data for tiles, todo: pass in
        let bordered_tile = Tile::new(|| {
            let mut img = im::RgbaImage::new(TILE_SIZE as u32, TILE_SIZE as u32);

                for x in 0..TILE_SIZE as u32 - 1 {
                    for y in 0..TILE_SIZE as u32 - 1 {
                        img.put_pixel(x, y, im::Rgb([255, 255, 255]).to_rgba());
                    }
                }
                for x in 0..TILE_SIZE as u32 {
                    img.put_pixel(x, 0, im::Rgb([0, 0, 0]).to_rgba());
                }
                for y in 0..TILE_SIZE as u32 {
                    img.put_pixel(0, y, im::Rgb([0, 0, 0]).to_rgba());
                }

                img
        }, false);

        let black_square = Tile::new(|| {
            let mut img = im::RgbaImage::new(TILE_SIZE as u32, TILE_SIZE as u32);

            for x in 0..TILE_SIZE as u32 {
                for y in 0..TILE_SIZE as u32 {
                    img.put_pixel(x, y, im::Rgb([0, 0, 0]).to_rgba());
                }
            }
            img
        }, true);

        let cross = Tile::new(|| {
            let mut img = im::RgbaImage::new(TILE_SIZE as u32, TILE_SIZE as u32);

            for x in 15..=17 {
                for y in 8..24 {
                    img.put_pixel(x, y, im::Rgb([255, 0, 0]).to_rgba());
                    img.put_pixel(y, x, im::Rgb([255, 0, 0]).to_rgba());
                }
            }
            img
        }, false);

        let tiles = vec![
            cross.clone(), bordered_tile.clone(),
            bordered_tile.clone(), bordered_tile.clone(),
            bordered_tile.clone(), bordered_tile.clone(),
            black_square.clone(), black_square.clone(),
            black_square.clone(), black_square.clone()
        ];

        if tiles.len() % width != 0 {
            panic!("The map is not rectangular!!");
        }
        
        TileMap {
            tiles: tiles,
            width: width
        }
    }

    /*
    TODO: implement
    Return true if all tiles intersecting the given Rect are permeable.
    Otherwise, return false if any tiles intersecting are impermeable.
    NB: note the assumption from position()
    */
    pub fn is_permeable_at_location(&self, r: &Rect) -> bool {
        // get all tiles overlapped by the given rect
        

        // determine permeability
        true
    }

    fn get_width(&self) -> u32 {
        return self.width as u32;
    }

    fn get_height(&self) -> u32 {
        return (self.tiles.len() / self.width) as u32;
    }

    /*
    x: x coordinate in pixels from origin
    y: y coordinate in pixels from origin
    return array index of tile at given coordinates
    */
    fn get_tile_index_at_coordinate(&self, x: u32, y: u32) -> u32 {
        let x1 = x / TILE_SIZE;        // tile number on the x
        let y1 = y / TILE_SIZE;        // tile number on the y
        return (y1 * self.get_width()) + (x1 % self.get_width());
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
        /*return Some(Rect{x: 50.0, y: 50.0,
            w: (self.get_width() * TILE_SIZE) as f64,
            h: (self.get_height() * TILE_SIZE) as f64});*/

        // for now, assume one tilemap over size of screen
        Some(Rect{
            x: 0,
            y: 0,
            w: SCREEN_WIDTH as i64,
            h: SCREEN_HEIGHT as i64
        })
    }

    fn update(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tile_index_at_coordinate() {
        let tile_map = TileMap::new(2);

        assert_eq!(0, tile_map.get_tile_index_at_coordinate(1, 1));
        assert_eq!(1, tile_map.get_tile_index_at_coordinate(35, 1));
        assert_eq!(2, tile_map.get_tile_index_at_coordinate(1, 35));
        assert_eq!(3, tile_map.get_tile_index_at_coordinate(35, 35));
        assert_eq!(4, tile_map.get_tile_index_at_coordinate(1, 70));
        //assert_eq!(1, tile_map.get_tile_at_coordinate(1+(TILE_SIZE as u32),1));
        //assert_eq!(2, tile_map.get_tile_at_coordinate(1,1+(TILE_SIZE as u32)));

        // todo: test edge cases
    }
}