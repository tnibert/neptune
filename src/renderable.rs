pub trait Renderable {
    fn render(&self) -> &im::RgbaImage;
    fn position(&self) -> (f64, f64);
}