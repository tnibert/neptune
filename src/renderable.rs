pub trait Renderable {
    // todo: should this have another name?
    // may not actually be rendering, likely returning prerendered image
    fn render(&self) -> &im::RgbaImage;
    fn position(&self) -> (f64, f64);
}