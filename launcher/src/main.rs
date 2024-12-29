use std::fmt::Debug;

use anyhow::Result;
use async_trait::async_trait;
use macroquad::prelude::*;

#[cfg(not(feature = "hotreload"))]
use game::caffeinated_gorilla::space::types::{GameColor, Position};

#[cfg(feature = "hotreload")]
use crate::hotreload::binding::caffeinated_gorilla::space::types::{GameColor, Key, Position};

mod input;
use input::*;

mod screen;
pub use screen::GameScreen;

mod texture_cache;

#[cfg(feature = "hotreload")]
mod hotreload;

#[cfg(feature = "hotreload")]
use crate::hotreload::binding::{
    caffeinated_gorilla::space::types::{KeyboardInfo, MouseInfo},
    WebAssemblyContext, WebAssemblyInstance,
};

#[cfg(not(feature = "hotreload"))]
pub use game::{
    exports::caffeinated_gorilla::space::game_api::{KeyboardInfo, MouseInfo},
    Game,
};

use texture_cache::TextureCache;

#[async_trait]
pub trait RunnableGameInstance: Send + Sync {
    fn update_frame(&self, mouse: MouseInfo, key: KeyboardInfo, screen: GameScreen);
    fn render_frame(&self, screen: GameScreen);
    fn save(&self) -> String;
}

#[cfg(not(feature = "hotreload"))]
#[async_trait]
impl RunnableGameInstance for Game {
    fn update_frame(&self, mouse: MouseInfo, key: KeyboardInfo, screen: GameScreen) {
        Game::update_frame(self, mouse, key, &screen, get_frame_time())
    }

    fn render_frame(&self, screen: GameScreen) {
        Game::render_frame(self, &screen)
    }

    fn save(&self) -> String {
        String::from_utf8(Game::save(&self)).unwrap_or_default()
    }
}

#[derive(Debug, Default)]
struct DebugState {
    pub skip_update: bool,
}

impl DebugState {
    pub fn toggle_skip_update(&mut self) {
        self.skip_update = !self.skip_update;
    }
}

async fn run_frame<R: RunnableGameInstance>(
    instance: &R,
    screen: GameScreen,
    debug: Option<&mut DebugState>,
) {
    let mouse = get_mouse_state();
    let key = get_key_info();

    // Inserts hot reload keys to skip update frame
    // Save and dump state to console
    let mut skip_update = false;

    if cfg!(debug_assertions) {
        if let Some(debug) = debug {
            skip_update = debug.skip_update;

            if key.pressed.contains(&Key::F1) {
                debug.toggle_skip_update();
            }
            if key.pressed.contains(&Key::F2) {
                println!("{}", instance.save());
            }
        }
    }

    if !skip_update {
        instance.update_frame(mouse, key, screen.clone());
    }
    instance.render_frame(screen.clone());

    screen.flush_image_draws().await;

    if skip_update {
        screen.draw_text(
            "Skip Update",
            Position {
                x: (screen.width() / 2.0) - 50.0,
                y: 30.0,
            },
            20,
            GameColor {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
        );
    }

    next_frame().await
}

#[cfg(not(feature = "hotreload"))]
async fn run(font: Font, texture_cache: TextureCache) -> Result<()> {
    let screen = GameScreen::new(font, texture_cache);
    let instance = Game::new(&screen);
    loop {
        run_frame(&instance, screen.clone(), None).await;
    }
}

#[cfg(feature = "hotreload")]
async fn run(font: Font, texture_cache: TextureCache) -> Result<()> {
    let context = WebAssemblyContext::load()?;
    let mut assembly = WebAssemblyInstance::load(context)?;

    let screen = GameScreen::new(font, texture_cache);

    let mut instance = assembly.create_game_instance(screen.clone())?;

    let file_watcher = crate::hotreload::watcher::FileWatcher::new(crate::hotreload::wasm_path()?)?;
    let mut debug_state = DebugState::default();

    loop {
        if file_watcher.changed() {
            let save_data = instance.save();
            let context = WebAssemblyContext::load()?;
            assembly = WebAssemblyInstance::load(context)?;
            instance = assembly.create_game_instance(screen.clone())?;
            if let Ok(save_data) = save_data {
                let _ = instance.load(save_data);
            }
        }

        run_frame(&instance, screen.clone(), Some(&mut debug_state)).await;
    }
}

#[macroquad::main("Rusty Space Shooter")]
async fn main() -> Result<()> {
    let font = load_ttf_font_from_bytes(include_bytes!("../../resources/Kreon-Regular.ttf"))
        .expect("Unable to load font");
    let texture_cache = TextureCache::default();

    run(font, texture_cache).await
}
