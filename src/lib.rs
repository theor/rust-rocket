//! This is the rust-rocket crate.
//! It is designed to work as a client library for GNU Rocket.

#[cfg(not(target_arch = "wasm32"))]
pub mod client;
pub mod interpolation;
pub mod player;
pub mod track;

#[cfg(not(target_arch = "wasm32"))]
pub use client::RocketClient;
pub use player::RocketPlayer;
