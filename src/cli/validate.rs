use crate::config::{load_config, validate_config};
use crate::error::Result;
use std::path::Path;

pub fn validate(file: &str) -> Result<()> {
    println!("Validating config: {}", file);
    let config = load_config(Path::new(file))?;
    validate_config(&config)?;
    println!("✓ Config is valid.");
    Ok(())
}
