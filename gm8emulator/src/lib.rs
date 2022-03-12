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
    Game, PlayType,
};
use jsutils::JsWaiter;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

const EXIT_SUCCESS: i32 = 0;
const EXIT_FAILURE: i32 = 1;

// fn main() {
//     let data = {
//         use std::io::prelude::*;
//         let mut file = std::fs::File::open("test/assets.map")
//             .expect("Failed to open file");
//         let mut buffer = Vec::<u8>::new();
//         file.read_to_end(&mut buffer)
//             .expect("Failed to read file");
//         buffer
//     };
//     let code = run(&data[..]);
//     process::exit(code);
// }

use std::{sync::Arc, future::Future, pin::Pin};

pub async fn run(
    data: &[u8],
    log: Arc<dyn Fn(&str)>,
    waiter: JsWaiter,
    ctx: web_sys::CanvasRenderingContext2d,
    on_pressed: Arc<dyn Fn() -> JsValue>,
    on_released: Arc<dyn Fn() -> JsValue>,
    js_audio: Arc<dyn jsutils::Audio>,
) -> i32 {
    let spoof_time = false; // !matches.opt_present("r");
    let frame_limiter = true; // !matches.opt_present("l");
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
    let encoding = encoding_rs::SHIFT_JIS; // TODO: argument
    let play_type = PlayType::Normal;
    let mut components =
        match Game::launch(
            assets,
            // absolute_path,
            game_args,
            // temp_dir,
            encoding,
            frame_limiter,
            play_type,
            Arc::clone(&log),
            waiter,
            ctx,
            on_pressed,
            on_released,
            js_audio,
        ).await {
            Ok(g) => g,
            Err(e) => {
                eprintln!("Failed to launch game: {}", e);
                return EXIT_FAILURE
            },
        };
    let time_now = gml::datetime::now_as_nanos();
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

// let strict = false; // matches.opt_present("s");
// let multithread = true; // !matches.opt_present("t");
// #[rustfmt::skip]
// let assets = gm8exe::reader::from_exe(
//     &mut file,                              // mut exe: AsRef<[u8]>
//     None,
//     strict,                                 // strict: bool
//     multithread,                            // multithread: bool
// );
// let assets = match assets {
//     Ok(assets) => assets,
//     Err(err) => {
//         eprintln!("failed to load '{}' - {}", input, err);
//         return EXIT_FAILURE
//     },
// };
// let bin_assets = bincode::serialize(&assets).expect("failed to serialize assets");
// let compressed = {
//     let mut e = flate2::write::ZlibEncoder::new(
//         Vec::new(),
//         flate2::Compression::best(),
//     );
//     e.write_all(&bin_assets[..]).expect("failed to compress assets");
//     e.finish().expect("Failed to compress")
// };
// {
//     use std::fs::File;
//     let mut file = File::create("test/assets.map")
//         .expect("Failed to create file");
//     // Write a slice of bytes to the file
//     file.write_all(
//         &compressed[..],
//     ).expect("Failed to write bytes");
// }
