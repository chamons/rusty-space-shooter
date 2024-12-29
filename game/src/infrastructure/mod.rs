#[cfg(feature = "hotreload")]
mod hotreload;
#[cfg(feature = "hotreload")]
pub use hotreload::{GameGuest, GameScreen, Shader};
#[cfg(feature = "hotreload")]
pub type Screen = GameScreen;

#[cfg(not(feature = "hotreload"))]
mod direct;
#[cfg(not(feature = "hotreload"))]
use direct::{GameScreen, GameShader};
#[cfg(not(feature = "hotreload"))]
pub type Screen = GameScreen;
#[cfg(not(feature = "hotreload"))]
pub type Shader = GameShader;

#[cfg(not(feature = "hotreload"))]
pub use direct::{GameScreenInterface, ShaderInterface, TextDimensions};
