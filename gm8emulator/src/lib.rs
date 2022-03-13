mod action;
mod asset;
mod game;
mod gml;
mod handleman;
pub mod input;
mod instance;
mod instancelist;
mod math;
mod render;
mod tile;
mod types;
mod util;
#[macro_use]
pub mod external;

use game::{
    Game,
    PlayType,
};

const EXIT_SUCCESS: i32 = 0;
const EXIT_FAILURE: i32 = 1;

pub async fn run(data: &[u8]) -> i32 {
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
        ).await {
            Ok(g) => g,
            Err(e) => {
                ext_elog!("Failed to launch game: {}", e);
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
            ext_log!("Runtime error: {}", err);
            EXIT_FAILURE
        },
    }
}
