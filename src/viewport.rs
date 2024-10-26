use crate::collision::{Rect, convert_world_coord_to_screen_coord};
use crate::game::{SCREEN_WIDTH, SCREEN_HEIGHT};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Viewport {
    viewport: Rect
}

impl Viewport {
    pub fn new_default() -> Viewport {
        Self {
            viewport: Rect{x: 0, y: 0, w: SCREEN_WIDTH as i64, h: SCREEN_HEIGHT as i64}
        }
    }

    pub fn new(r: Rect) -> Viewport {
        Self {
            viewport: r
        }
    }

    pub fn screen_coordinates(&self, world_position: &Rect) -> Rect {
        return convert_world_coord_to_screen_coord(world_position, &self.viewport);
    }

    pub fn get(&self) -> Rect {
        return self.viewport.clone()
    }
}
