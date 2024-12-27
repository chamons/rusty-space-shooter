#[cfg(feature = "hotreload")]
mod hotreload;
#[cfg(feature = "hotreload")]
pub use hotreload::{GameGuest, GameScreen};
#[cfg(feature = "hotreload")]
pub type Screen = GameScreen;

#[cfg(not(feature = "hotreload"))]
mod direct;
#[cfg(not(feature = "hotreload"))]
use direct::GameScreen;
#[cfg(not(feature = "hotreload"))]
pub type Screen = GameScreen;
#[cfg(not(feature = "hotreload"))]
pub use direct::GameScreenInterface;
