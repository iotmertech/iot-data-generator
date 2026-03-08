use crate::config::{load_config, validate_config};
use crate::device::DevicePool;
use crate::error::Result;
use crate::payload::PayloadGenerator;
use std::path::Path;

pub fn preview(file: &str, count: usize) -> Result<()> {
    let config = load_config(Path::new(file))?;
    validate_config(&config)?;

    let generator = PayloadGenerator::from_config(&config)?;
    let mut pool = DevicePool::new(config.device.count, &config.device.id_prefix);

    println!("Previewing {} payload(s) from '{}':", count, file);
    println!();

    for i in 0..count {
        let device = pool.next().clone();
        let payload = generator.generate_preview(&device, i)?;
        println!("--- Sample {} (device: {}) ---", i + 1, device.device_id);
        println!("{}", payload);
        println!();
    }

    Ok(())
}
