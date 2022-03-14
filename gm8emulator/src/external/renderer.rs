pub use crate::render::{
    atlas,
};

pub trait Renderer {
    fn clear(&self);
    fn load_sprites(&mut self, atl: atlas::AtlasBuilder);
    fn draw_rect(&self, x: f64, y: f64, w: f64, h: f64);
}
