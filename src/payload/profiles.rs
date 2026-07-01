use crate::device::DeviceContext;
use crate::error::Result;
use chrono::{DateTime, Utc};
use rand::Rng;
use serde_json::{json, Value};

/// Generate a random IoT payload for the given device context.
///
/// `ts` is the timestamp for this message (from the simulated clock) and
/// `ts_field` is the JSON key it is written under.
pub fn generate_random(
    device: &DeviceContext,
    seq: usize,
    ts: DateTime<Utc>,
    ts_field: &str,
) -> Result<Value> {
    let mut rng = rand::thread_rng();
    let mut payload = json!({
        "device_id": device.device_id,
        "device_index": device.index,
        "seq": seq,
        "temperature": round2(rng.gen_range(15.0_f64..45.0)),
        "humidity": round2(rng.gen_range(20.0_f64..95.0)),
        "voltage": round2(rng.gen_range(220.0_f64..240.0)),
        "current": round2(rng.gen_range(0.1_f64..15.0)),
        "power": round2(rng.gen_range(10.0_f64..3600.0)),
        "energy_total": round2(rng.gen_range(0.0_f64..10000.0)),
        "status": random_status(&mut rng),
    });
    if let Some(obj) = payload.as_object_mut() {
        obj.insert(ts_field.to_string(), json!(ts.to_rfc3339()));
    }
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

    fn gen(device: &DeviceContext, seq: usize) -> serde_json::Value {
        generate_random(device, seq, Utc::now(), "ts").unwrap()
    }

    #[test]
    fn test_generate_random_has_required_fields() {
        let device = make_device(0);
        let val = gen(&device, 1);
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
        let val = gen(&device, 0);
        assert_eq!(val["device_id"].as_str().unwrap(), "device-0003");
        assert_eq!(val["device_index"].as_u64().unwrap(), 3);
    }

    #[test]
    fn test_generate_random_seq_matches() {
        let device = make_device(0);
        let val = gen(&device, 99);
        assert_eq!(val["seq"].as_u64().unwrap(), 99);
    }

    #[test]
    fn test_generate_random_temperature_in_range() {
        let device = make_device(0);
        for _ in 0..50 {
            let val = gen(&device, 0);
            let temp = val["temperature"].as_f64().unwrap();
            assert!(
                (15.0..45.0).contains(&temp),
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
            let val = gen(&device, 0);
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
