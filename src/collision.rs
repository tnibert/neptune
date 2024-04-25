// todo: wait a sec, a rect can't have negative width and height...
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Rect {
    pub x: i64,
    pub y: i64,
    pub w: i64,         // width
    pub h: i64          // height
}

pub fn new_point(x: i64, y: i64) -> Rect {
    Rect { x: x, y: y, w: 0, h: 0 }
}

// provided world coordinates and a visibility window,
// return the screen coordinates
pub fn convert_world_coord_to_screen_coord(world_coord: &Rect, window: &Rect) -> Rect {
    Rect{
        x: world_coord.x - window.x,
        y: world_coord.y - window.y,
        w: world_coord.w,
        h: world_coord.h
    }
}

/*
Detect overlapping rectangles
:param rect1:
:param rect2:
:return: True if collided, False if not
*/
fn collide(rect1: Rect, rect2: Rect) -> bool {
    return rect1.x < rect2.x + rect2.w &&
           rect1.x + rect1.w > rect2.x &&
           rect1.y < rect2.y + rect2.h && 
           rect1.y + rect1.h > rect2.y;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_world_coord_to_screen_identity() {
        let world_coord = Rect{x: 100, y: 100, w: 20, h: 20};
        let window = Rect{x: 0, y: 0, w: 640, h: 480};

        let test = convert_world_coord_to_screen_coord(&world_coord, &window);

        assert_eq!(test, Rect{x: 100, y: 100, w: 20, h: 20});
    }

    #[test]
    fn test_convert_world_coord_to_screen_offset() {
        let offset = 80;
        let world_coord = Rect{x: 100, y: 100, w: 20, h: 20};
        let window = Rect{x: 0+offset, y: 0+offset, w: 640+offset, h: 480+offset};

        let test = convert_world_coord_to_screen_coord(&world_coord, &window);

        assert_eq!(test, Rect{x: 100-offset, y: 100-offset, w: 20, h: 20});
    }
}
