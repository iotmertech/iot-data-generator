use crate::device::DeviceContext;
use crate::error::Result;
use chrono::Utc;
use rand::Rng;
use serde_json::{json, Value};

/// Generate a random IoT payload for the given device context.
pub fn generate_random(device: &DeviceContext, seq: usize) -> Result<Value> {
    let mut rng = rand::thread_rng();
    let payload = json!({
        "device_id": device.device_id,
        "device_index": device.index,
        "seq": seq,
        "ts": Utc::now().to_rfc3339(),
        "temperature": round2(rng.gen_range(15.0_f64..45.0)),
        "humidity": round2(rng.gen_range(20.0_f64..95.0)),
        "voltage": round2(rng.gen_range(220.0_f64..240.0)),
        "current": round2(rng.gen_range(0.1_f64..15.0)),
        "power": round2(rng.gen_range(10.0_f64..3600.0)),
        "energy_total": round2(rng.gen_range(0.0_f64..10000.0)),
        "status": random_status(&mut rng),
    });
    Ok(payload)
}

fn round2(val: f64) -> f64 {
    (val * 100.0).round() / 100.0
}

fn random_status(rng: &mut impl Rng) -> &'static str {
    let statuses = ["online", "online", "online", "online", "offline", "error"];
    statuses[rng.gen_range(0..statuses.len())]
}
