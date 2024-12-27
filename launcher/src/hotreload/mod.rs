use std::path::PathBuf;

use anyhow::Result;

pub mod binding;
pub mod watcher;

pub fn wasm_path() -> Result<PathBuf> {
    let mut wasm_path = std::env::current_exe()?;
    wasm_path.pop();
    wasm_path.push("game.wasm");
    Ok(wasm_path)
}
