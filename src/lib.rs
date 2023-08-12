//! This is the rust-rocket crate.
//! It is designed to work as a client library for GNU Rocket.


#[cfg(feature = "client")]
pub mod client;
pub mod interpolation;
pub mod player;
pub mod track;


#[cfg(feature = "client")]
pub use client::RocketClient;
pub use player::RocketPlayer;
