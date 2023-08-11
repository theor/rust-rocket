//! This module contains a barebones player.
use crate::track::{Track, RocketEngine};

/// A player for tracks dumped by
/// [`RocketClient::save_tracks`](crate::RocketClient::save_tracks).
///
/// # Examples
///
/// ```rust,no_run
/// # use rust_rocket::RocketClient;
/// # use rust_rocket::RocketPlayer;
/// let client = RocketClient::new().unwrap();
/// // ...
/// // Run the demo and edit your sync tracks, then call save_tracks
/// // ...
/// let tracks = client.save_tracks();
/// // ...
/// // Serialize tracks to a file (see examples/edit.rs)
/// // And deserialize from a file in your release build (examples/play.rs)
/// // ...
/// let player = RocketPlayer::new(tracks);
/// println!("Value at row 123: {}", player.get_track("test").unwrap().get_value(123.));
/// ```
pub struct RocketPlayer {
    tracks: Vec<Track>,
}


impl RocketEngine for RocketPlayer {

    fn get_track_index(&self, name: &str) -> Option<usize> {
        self.tracks.iter().enumerate().find(|t| t.1.get_name() == name).map(|t| t.0)
    }
    fn get_track(&self, index: usize) ->&Track { &self.tracks[index] }
}

impl RocketPlayer {
    /// Constructs a `RocketPlayer` from `Track`s.
    pub fn new(tracks: Vec<Track>) -> Self {
        // Convert to a HashMap for perf (not benchmarked)
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

        assert_eq!(player.get_track("test1").unwrap().get_value(0.), 1.0);
        assert_eq!(player.get_track("test2").unwrap().get_value(0.), 2.0);
    }

    #[test]
    fn no_surprise_tracks() {
        let tracks = get_test_tracks();
        let player = RocketPlayer::new(tracks);
        assert!(player
            .get_track("hello this track should not exist")
            .is_none());
    }
}
