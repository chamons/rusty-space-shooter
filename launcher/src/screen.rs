use std::sync::Arc;

use macroquad::{
    color::{Color, WHITE},
    math::Vec2,
    shapes::draw_line,
    text::{draw_text_ex, Font, TextParams},
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
};

use crate::texture_cache::TextureCache;

#[cfg(not(feature = "hotreload"))]
use game::example::game::types::{GameColor, Position, Size};

#[cfg(feature = "hotreload")]
use crate::hotreload::binding::example::game::types::{GameColor, Position, Size};

struct ImageRenderRequest {
    filename: String,
    position: Position,
    size: Option<Size>,
}

#[derive(Clone)]
pub struct GameScreen {
    font: Font,
    texture_cache: Arc<async_mutex::Mutex<TextureCache>>,
    image_requests: Arc<std::sync::Mutex<Vec<ImageRenderRequest>>>,
}

impl GameScreen {
    pub fn new(font: Font, texture_cache: TextureCache) -> Self {
        Self {
            font,
            texture_cache: Arc::new(async_mutex::Mutex::new(texture_cache)),
            image_requests: Arc::new(std::sync::Mutex::new(vec![])),
        }
    }

    async fn fetch_texture(&self, filename: &str) -> Option<Texture2D> {
        let mut texture_cache = self.texture_cache.lock().await;
        texture_cache.get(filename).await.ok()
    }

    pub fn draw_text(&self, text: &str, position: Position, size: f32, color: GameColor) {
        draw_text_ex(
            text,
            position.x,
            position.y,
            TextParams {
                font: Some(&self.font),
                font_size: size as u16,
                color: Color {
                    r: color.r,
                    g: color.g,
                    b: color.b,
                    a: color.a,
                },
                ..Default::default()
            },
        );
    }

    // Make this store a list and flush after frame to prevent needing async in render
    pub fn draw_image(&self, filename: &str, position: Position, size: Option<Size>) {
        self.image_requests
            .lock()
            .unwrap()
            .push(ImageRenderRequest {
                filename: filename.to_string(),
                position,
                size,
            });
    }

    pub async fn flush_image_draws(&self) {
        let images: Vec<ImageRenderRequest> = {
            let mut image_requests = self.image_requests.lock().unwrap();
            image_requests.drain(..).collect()
        };
        for image in images {
            // Ignore image loading errors and just skip render
            if let Some(texture) = self.fetch_texture(&image.filename).await {
                let mut params = DrawTextureParams::default();
                if let Some(size) = image.size {
                    params.dest_size = Some(Vec2 {
                        x: size.width,
                        y: size.height,
                    })
                }
                draw_texture_ex(&texture, image.position.x, image.position.y, WHITE, params);
            }
        }
    }

    pub fn draw_line(&self, first: Position, second: Position, thickness: f32, color: GameColor) {
        draw_line(
            first.x,
            first.y,
            second.x,
            second.y,
            thickness,
            Color {
                r: color.r,
                g: color.g,
                b: color.b,
                a: color.a,
            },
        )
    }
}

#[cfg(not(feature = "hotreload"))]
#[async_trait::async_trait]
impl game::GameScreenInterface for GameScreen {
    fn draw_text(&self, text: &str, position: Position, size: f32, color: GameColor) {
        self.draw_text(text, position, size, color);
    }

    fn draw_image(&self, filename: &str, position: Position, size: Option<Size>) {
        self.draw_image(filename, position, size);
    }

    fn draw_line(&self, first: Position, second: Position, thickness: f32, color: GameColor) {
        self.draw_line(first, second, thickness, color);
    }
}
