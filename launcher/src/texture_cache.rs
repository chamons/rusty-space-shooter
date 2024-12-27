use std::collections::HashMap;

use anyhow::Result;
use macroquad::texture::{load_texture, Texture2D};

#[derive(Debug, Default)]
pub struct TextureCache {
    textures: HashMap<String, Texture2D>,
}

impl TextureCache {
    pub async fn get(&mut self, filename: &str) -> Result<Texture2D> {
        if let Some(texture) = self.textures.get(filename) {
            Ok(texture.clone())
        } else {
            let texture = load_texture(filename).await?;
            self.textures.insert(filename.to_string(), texture.clone());
            Ok(texture)
        }
    }
}
