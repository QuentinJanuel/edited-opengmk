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

use game::{
    Game, PlayType,
};
use std::process;

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

use std::sync::Arc;

pub fn run(
    data: &[u8],
    log: Arc<dyn Fn(&str)>,
) -> i32 {
    log("Initializing game");
    let spoof_time = true; // !matches.opt_present("r");
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
    log("Successfully loaded assets!");
    let encoding = encoding_rs::SHIFT_JIS; // TODO: argument
    let play_type = PlayType::Normal;
    log("Lauching game...");
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
        ) {
            Ok(g) => g,
            Err(e) => {
                eprintln!("Failed to launch game: {}", e);
                return EXIT_FAILURE
            },
        };
    log("Successfully launched game!");
    log("Getting time_now...");
    let time_now = gml::datetime::now_as_nanos();
    log("Successfully got time_now!");
    log("Running game loop...");
    let result = {
        components.spoofed_time_nanos = if spoof_time { Some(time_now) } else { None };
        components.run()
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
