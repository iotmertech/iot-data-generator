use crate::config::{load_config, validate_config};
use crate::device::DevicePool;
use crate::error::Result;
use crate::payload::{PayloadGenerator, SimClock};
use std::path::Path;

pub fn preview(file: &str, count: usize) -> Result<()> {
    let config = load_config(Path::new(file))?;
    validate_config(&config)?;

    let generator = PayloadGenerator::from_config(&config)?;
    let mut pool = DevicePool::new(config.device.count, &config.device.id_prefix);
    let mut clock = SimClock::from_config(&config.time)?;
    let ts_field = clock.field().to_string();

    println!("Previewing {} payload(s) from '{}':", count, file);
    println!();

    for i in 0..count {
        let device = pool.next().clone();
        let ts = clock.timestamp(i);
        let payload = generator.generate_preview(&device, i, ts, &ts_field)?;
        println!("--- Sample {} (device: {}) ---", i + 1, device.device_id);
        println!("{}", payload);
        println!();
    }

    Ok(())
}
