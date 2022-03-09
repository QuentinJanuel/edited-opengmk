use wasm_bindgen::prelude::*;
use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;

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
