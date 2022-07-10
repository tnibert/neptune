use crate::renderable::Renderable;
use crate::tile::Tile;

struct TileArea {
    width: usize,
    height: usize,
    map: Vec<u8>,
    available_tiles: Vec<Tile>
}

impl TileArea {
    fn new(width: usize) -> Self {
        let available_tiles = vec![Tile::new(), Tile::new()];

        let tile_map = vec![0, 0, 0,
                            0, 0, 0,
                            1, 0, 1];

        if tile_map.len() % width != 0 {
            panic!("The map is not rectangular!!");
        }
        let height = tile_map.len()/width;

        TileArea {
            width: width,
            height: height,
            map: tile_map,
            available_tiles: available_tiles
        }
        
    }
}

/*impl Renderable for TileArea {
    fn render(&self) -> &im::RgbaImage {

    }
}*/

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let test = TileArea::new(3);
    }
}