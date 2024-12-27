use anyhow::Result;
use async_trait::async_trait;
use macroquad::prelude::*;

mod input;
use input::*;

mod screen;
pub use screen::GameScreen;

mod texture_cache;

#[cfg(feature = "hotreload")]
mod hotreload;

#[cfg(feature = "hotreload")]
use crate::hotreload::binding::{
    example::game::types::{KeyboardInfo, MouseInfo},
    WebAssemblyContext, WebAssemblyInstance,
};

#[cfg(not(feature = "hotreload"))]
pub use game::{
    exports::example::game::game_api::{KeyboardInfo, MouseInfo},
    Game,
};

use texture_cache::TextureCache;

#[async_trait]
pub trait RunnableGameInstance: Send + Sync {
    fn run_frame(&self, mouse: MouseInfo, key: KeyboardInfo, screen: GameScreen);
}

#[cfg(not(feature = "hotreload"))]
#[async_trait]
impl RunnableGameInstance for Game {
    fn run_frame(&self, mouse: MouseInfo, key: KeyboardInfo, screen: GameScreen) {
        Game::run_frame(self, mouse, key, &screen)
    }
}

async fn run_frame<R: RunnableGameInstance>(instance: &R, screen: GameScreen) {
    let mouse = get_mouse_state();
    let key = get_key_info();
    instance.run_frame(mouse, key, screen.clone());

    screen.flush_image_draws().await;

    next_frame().await
}

#[cfg(not(feature = "hotreload"))]
async fn run(font: Font, texture_cache: TextureCache) -> Result<()> {
    let instance = Game::new();
    let screen = GameScreen::new(font, texture_cache);
    loop {
        run_frame(&instance, screen.clone()).await;
    }
}

#[cfg(feature = "hotreload")]
async fn run(font: Font, texture_cache: TextureCache) -> Result<()> {
    let context = WebAssemblyContext::load()?;
    let mut assembly = WebAssemblyInstance::load(context)?;
    let mut instance = assembly.create_game_instance()?;

    let file_watcher = crate::hotreload::watcher::FileWatcher::new(crate::hotreload::wasm_path()?)?;
    let screen = GameScreen::new(font, texture_cache);

    loop {
        if file_watcher.changed() {
            let save_data = instance.save();
            let context = WebAssemblyContext::load()?;
            assembly = WebAssemblyInstance::load(context)?;
            instance = assembly.create_game_instance()?;
            if let Ok(save_data) = save_data {
                let _ = instance.load(save_data);
            }
        }

        run_frame(&instance, screen.clone()).await;
    }
}

#[macroquad::main("Rust Hotreload Example")]
async fn main() -> Result<()> {
    let font = load_ttf_font_from_bytes(include_bytes!("../../resources/Kreon-Regular.ttf"))
        .expect("Unable to load font");
    let texture_cache = TextureCache::default();

    run(font, texture_cache).await
}
