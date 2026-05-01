use anyhow::{Context, Result};
use serde::{de::DeserializeOwned, Serialize};
use std::fs;
use std::path::Path;

pub fn load<T: DeserializeOwned + Default>(path: &Path) -> Result<T> {
    if !path.exists() {
        return Ok(T::default());
    }
    let data = fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;
    if data.trim().is_empty() {
        return Ok(T::default());
    }
    let value = serde_json::from_str(&data).with_context(|| format!("Failed to parse {}", path.display()))?;
    Ok(value)
}

pub fn save<T: Serialize>(path: &Path, data: &T) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("Failed to create directory {}", parent.display()))?;
    }
    let json = serde_json::to_string_pretty(data)?;
    fs::write(path, json).with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(())
}
