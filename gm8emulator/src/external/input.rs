pub trait Input {
    fn pressed(&self) -> Vec<crate::input::Button>;
    fn released(&self) -> Vec<crate::input::Button>;
}
