wit_bindgen::generate!({
    world: "hotreload-example",
    path: "../wit",
});

mod colors;
pub use colors::*;

mod ui;

mod game;
pub use game::Game;

mod infrastructure;
pub use infrastructure::*;

#[cfg(feature = "hotreload")]
export!(GameGuest);
