pub use crate::render::{
    atlas,
};
pub use crate::types::Colour;

pub trait Renderer {
    fn clear(&self, colour: Colour);
    fn load_sprites(&self, atl: atlas::AtlasBuilder);
    fn get_rect(&self, atl: atlas::AtlasRef) -> Option<atlas::AtlasRect>;
    fn draw_sprite_general(
        &self,
        texture: atlas::AtlasRef,
        part_x: f64,
        part_y: f64,
        part_w: f64,
        part_h: f64,
        x: f64,
        y: f64,
        xscale: f64,
        yscale: f64,
        angle: f64,
        col1: i32,
        col2: i32,
        col3: i32,
        col4: i32,
        alpha: f64,
        use_origin: bool,
    );
    fn set_view(
        &self,
        src_x: i32,
        src_y: i32,
        src_w: i32,
        src_h: i32,
        src_angle: f64,
        port_x: i32,
        port_y: i32,
        port_w: i32,
        port_h: i32,
    );
}
