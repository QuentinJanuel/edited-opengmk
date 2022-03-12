use std::{
    sync::Arc,
    pin::Pin,
    future::Future,
    time::Duration,
};

pub type Fut<T = ()> = Pin<Box<dyn Future<Output = T>>>;

pub trait Time {
    fn now_as_timestamp_nanos(&self) -> u128;
    fn wait(&self, dur: Duration) -> Fut;
}

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
