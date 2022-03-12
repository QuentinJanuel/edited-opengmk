mod action;
mod asset;
mod game;
mod gml;
mod handleman;
mod input;
mod instance;
mod instancelist;
mod math;
mod render;
mod tile;
mod types;
mod util;
pub mod jsutils;

use game::{
    Game,
    PlayType,
};
use wasm_bindgen::JsValue;
use std::sync::Arc;

const EXIT_SUCCESS: i32 = 0;
const EXIT_FAILURE: i32 = 1;


pub async fn run(
    data: &[u8],
    log: Arc<dyn Fn(&str)>,
    ctx: web_sys::CanvasRenderingContext2d,
    on_pressed: Arc<dyn Fn() -> JsValue>,
    on_released: Arc<dyn Fn() -> JsValue>,
    js_audio: Arc<dyn jsutils::Audio>,
    js_time: Arc<dyn jsutils::Time>,
) -> i32 {
    let spoof_time = false;
    let frame_limiter = true;
    let game_args = vec![String::new()];
    let uncompressed = {
        use std::io::prelude::*;
        use flate2::read::ZlibDecoder;
        let mut d = ZlibDecoder::new(data);
        let mut uncompressed = Vec::<u8>::new();
        d.read_to_end(&mut uncompressed).expect("Failed to decompress");
        uncompressed
    };
    let assets = bincode::deserialize(&uncompressed[..])
        .expect("failed to deserialize assets");
    let encoding = encoding_rs::SHIFT_JIS;
    let play_type = PlayType::Normal;
    let mut components =
        match Game::launch(
            assets,
            game_args,
            encoding,
            frame_limiter,
            play_type,
            Arc::clone(&log),
            ctx,
            on_pressed,
            on_released,
            js_audio,
            Arc::clone(&js_time),
        ).await {
            Ok(g) => g,
            Err(e) => {
                eprintln!("Failed to launch game: {}", e);
                return EXIT_FAILURE
            },
        };
    let time_now = gml::datetime::now_as_nanos(js_time);
    let result = {
        components.spoofed_time_nanos = if spoof_time { Some(time_now) } else { None };
        components.run().await
    };
    match result {
        Ok(()) => EXIT_SUCCESS,
        Err(err) => {
            println!("Runtime error: {}", err);
            log(&format!("Runtime error: {}", err));
            EXIT_FAILURE
        },
    }
}
