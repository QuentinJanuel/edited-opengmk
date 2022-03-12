use std::{sync::Arc, pin::Pin, future::Future};
use wasm_bindgen::prelude::*;
use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;

pub type Fut<T = ()> = Pin<Box<dyn Future<Output = T>>>;

pub struct JsWaiter {
    waiter: js_sys::Function,
}

impl JsWaiter {
    pub fn new(waiter: js_sys::Function) -> Self {
        Self { waiter }
    }
    pub async fn wait(&self, duration: instant::Duration) -> Result<(), JsValue> {
        let this = JsValue::null();
        let seconds = duration.as_secs() as f64 + duration.subsec_nanos() as f64 / 1_000_000_000.0;
        let seconds = JsValue::from(seconds);
        let promise = self.waiter.call1(&this, &seconds)?;
        let promise = Promise::resolve(&promise);
        let future = JsFuture::from(promise);
        future.await?;
        Ok(())
    }
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
