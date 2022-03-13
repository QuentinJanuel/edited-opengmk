use std::sync::Arc;
use super::Fut;

pub struct Sound {
    pub id: i32,
    pub data: Arc<[u8]>,
}

pub trait Audio {
    fn load(&self, sounds: Vec<Sound>) -> Fut;
    fn play(&self, id: i32, loop_: bool);
    fn stop(&self, id: i32);
    fn stop_all(&self);
}
