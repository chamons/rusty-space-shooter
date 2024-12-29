wit_bindgen::generate!({
    world: "space-shooter-game",
    path: "../wit",
});

#[allow(dead_code)]
mod colors;

#[allow(dead_code)]
mod ui;

mod game;
pub use game::Game;

mod infrastructure;
pub use infrastructure::*;

mod state;

#[cfg(feature = "hotreload")]
export!(GameGuest);

#[allow(dead_code)]
mod math;
