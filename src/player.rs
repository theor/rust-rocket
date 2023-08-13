//! This module contains a barebones player.
use std::io::Cursor;

use byteorder::{ReadBytesExt, LE};

use crate::{
    interpolation::Interpolation,
    track::{Key, RocketEngine, Track},
};

/// A player for tracks dumped by
/// [`RocketClient::save_tracks`](crate::RocketClient::save_tracks).
///
/// # Examples
///
/// ```rust,no_run
/// # use rust_rocket::RocketPlayer;
/// # use crate::rust_rocket::track::RocketEngine;
/// // let client = RocketClient::new().unwrap();
/// // ...
/// // Run the demo and edit your sync tracks, then call save_tracks
/// // ...
/// // let tracks = client.serialize();
/// // ...
/// // Serialize tracks to a file (see examples/edit.rs)
/// // And deserialize from a file in your release build (examples/play.rs)
/// // ...
/// let player = RocketPlayer::deserialize(&[]);
/// println!("Value at row 123: {}", player.get_track(player.get_track_index("test").unwrap()).get_value(123.));
/// ```
pub struct RocketPlayer {
    tracks: Vec<Track>,
}

impl RocketEngine for RocketPlayer {
    fn get_track_index(&self, name: &str) -> Option<usize> {
        self.tracks
            .iter()
            .enumerate()
            .find(|t| t.1.get_name() == name)
            .map(|t| t.0)
    }
    fn get_track_index_mut(&mut self, name: &str) -> Result<usize, std::io::Error> {
        Ok(self.get_track_index(name).unwrap())
    }
    fn get_track(&self, index: usize) -> &Track {
        &self.tracks[index]
    }
}

impl RocketPlayer {
    /// Constructs a `RocketPlayer` from `Track`s.
    pub fn new(tracks: Vec<Track>) -> Self {
        // Convert to a HashMap for perf (not benchmarked)
        Self { tracks: tracks }
    }

    pub fn track_count(&self) -> usize {
        self.tracks.len()
    }

    pub fn deserialize(data: &[u8]) -> Self {
        let mut bytes = Cursor::new(data);
        // println!("{:?}", bytes);
        let track_count = bytes.read_u64::<LE>().unwrap();
        // println!("track count {track_count}");
        let mut tracks = Vec::with_capacity(track_count as usize);
        for _i in 0..track_count {
            let name_len = bytes.read_u64::<LE>().unwrap() as usize;
            let name = std::str::from_utf8(
                &bytes.get_ref()[bytes.position() as usize..bytes.position() as usize + name_len],
            )
            .unwrap();
            bytes.set_position(bytes.position() + name_len as u64);

            let key_count = bytes.read_u64::<LE>().unwrap() as usize;
            let mut t = Track::with_capacity(name, key_count as usize);
            for _k in 0..key_count {
                let row = bytes.read_u32::<LE>().unwrap();
                let value = bytes.read_f32::<LE>().unwrap();
                let interp: Interpolation = match bytes.read_u32::<LE>().unwrap() {
                    0 => Interpolation::Step,
                    1 => Interpolation::Linear,
                    2 => Interpolation::Smooth,
                    3 => Interpolation::Ramp,
                    _ => unreachable!(),
                };
                let key = Key::new(row, value, interp);
                t.set_key(key);
            }

            // println!("  name {name_len} {name} {key_count}");
            tracks.push(t);
            // let name = bytes.
        }
        Self { tracks: tracks }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpolation::Interpolation;
    use crate::track::Key;

    fn get_test_tracks() -> Vec<Track> {
        vec![
            {
                let mut track = Track::new("test1");
                track.set_key(Key::new(0, 1.0, Interpolation::Step));
                track.set_key(Key::new(5, 0.0, Interpolation::Step));
                track.set_key(Key::new(10, 1.0, Interpolation::Step));
                track
            },
            {
                let mut track = Track::new("test2");
                track.set_key(Key::new(0, 2.0, Interpolation::Step));
                track.set_key(Key::new(5, 0.0, Interpolation::Step));
                track.set_key(Key::new(10, 2.0, Interpolation::Step));
                track
            },
        ]
    }

    #[test]
    fn finds_all_tracks() {
        let tracks = get_test_tracks();
        let player = RocketPlayer::new(tracks);

        // Ugly repeated calls to get_track to reflect average use case :)

        assert_eq!(
            player
                .get_track(player.get_track_index("test1").unwrap())
                .get_value(0.),
            1.0
        );
        assert_eq!(
            player
                .get_track(player.get_track_index("test2").unwrap())
                .get_value(0.),
            2.0
        );
    }
}
