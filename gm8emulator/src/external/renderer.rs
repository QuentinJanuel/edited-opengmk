pub trait Renderer {
    fn clear(&self);
    fn draw_rect(&self, x: f64, y: f64, w: f64, h: f64);
}
