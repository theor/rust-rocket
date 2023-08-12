use rust_rocket::track::RocketEngine;
use rust_rocket::RocketPlayer;
use std::error::Error;
use std::fs::File;
use std::time::Duration;

static TRACKS_FILE: &str = "tracks.bin";

fn main() -> Result<(), Box<dyn Error>> {
    let rocket = {
        // Open previously saved file (see examples/edit.rs)
        let file = std::fs::read(TRACKS_FILE)?;
        // Deserialize from the file into Vec<Track> and
        // construct a new read-only, offline RocketPlayer
        RocketPlayer::deserialize(&file)
    };
    println!("Tracks loaded from {}", TRACKS_FILE);

    let mut current_row = 0;

    loop {
        println!(
            "value: {:?} (row: {:?})",
            rocket
                .get_track(rocket.get_track_index("test").unwrap())
                .get_value(current_row as f32),
            current_row
        );

        current_row += 1;
        std::thread::sleep(Duration::from_millis(32));
    }
}
