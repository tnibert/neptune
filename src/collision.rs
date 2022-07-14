pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,         // width
    pub h: f64          // height
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