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

pub fn round2(val: f64) -> f64 {
    (val * 100.0).round() / 100.0
}

fn random_status(rng: &mut impl Rng) -> &'static str {
    let statuses = ["online", "online", "online", "online", "offline", "error"];
    statuses[rng.gen_range(0..statuses.len())]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::context::DeviceContext;

    fn make_device(index: usize) -> DeviceContext {
        DeviceContext::new(index, "device")
    }

    #[test]
    fn test_generate_random_has_required_fields() {
        let device = make_device(0);
        let val = generate_random(&device, 1).unwrap();
        assert!(val.get("device_id").is_some());
        assert!(val.get("device_index").is_some());
        assert!(val.get("seq").is_some());
        assert!(val.get("ts").is_some());
        assert!(val.get("temperature").is_some());
        assert!(val.get("humidity").is_some());
        assert!(val.get("voltage").is_some());
        assert!(val.get("current").is_some());
        assert!(val.get("power").is_some());
        assert!(val.get("energy_total").is_some());
        assert!(val.get("status").is_some());
    }

    #[test]
    fn test_generate_random_device_id_matches() {
        let device = make_device(3);
        let val = generate_random(&device, 0).unwrap();
        assert_eq!(val["device_id"].as_str().unwrap(), "device-0003");
        assert_eq!(val["device_index"].as_u64().unwrap(), 3);
    }

    #[test]
    fn test_generate_random_seq_matches() {
        let device = make_device(0);
        let val = generate_random(&device, 99).unwrap();
        assert_eq!(val["seq"].as_u64().unwrap(), 99);
    }

    #[test]
    fn test_generate_random_temperature_in_range() {
        let device = make_device(0);
        for _ in 0..50 {
            let val = generate_random(&device, 0).unwrap();
            let temp = val["temperature"].as_f64().unwrap();
            assert!(
                temp >= 15.0 && temp < 45.0,
                "temperature out of range: {}",
                temp
            );
        }
    }

    #[test]
    fn test_generate_random_status_is_valid() {
        let device = make_device(0);
        let valid = ["online", "offline", "error"];
        for _ in 0..30 {
            let val = generate_random(&device, 0).unwrap();
            let status = val["status"].as_str().unwrap();
            assert!(valid.contains(&status), "unexpected status: {}", status);
        }
    }

    #[test]
    fn test_round2_precision() {
        // 1.005 in IEEE 754 is slightly less than 1.005, so it rounds to 1.00 — expected.
        assert_eq!(round2(1.0), 1.0);
        assert_eq!(round2(1.234), 1.23);
        assert_eq!(round2(0.0), 0.0);
        assert_eq!(round2(23.456), 23.46);
        assert_eq!(round2(99.999), 100.0);
    }
}
