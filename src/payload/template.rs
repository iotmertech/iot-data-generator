use crate::device::DeviceContext;
use crate::error::{MerError, Result};
use chrono::{DateTime, Utc};
use handlebars::Handlebars;
use rand::Rng;
use serde_json::json;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// Build a Handlebars registry with all supported helpers.
pub fn build_registry() -> Handlebars<'static> {
    let mut hbs = Handlebars::new();

    hbs.register_helper(
        "now_utc",
        Box::new(
            |_: &handlebars::Helper,
             _: &Handlebars,
             _: &handlebars::Context,
             _: &mut handlebars::RenderContext,
             out: &mut dyn handlebars::Output|
             -> handlebars::HelperResult {
                out.write(&Utc::now().to_rfc3339())?;
                Ok(())
            },
        ),
    );

    hbs.register_helper(
        "random_int",
        Box::new(
            |h: &handlebars::Helper,
             _: &Handlebars,
             _: &handlebars::Context,
             _: &mut handlebars::RenderContext,
             out: &mut dyn handlebars::Output|
             -> handlebars::HelperResult {
                let min = h.param(0).and_then(|v| v.value().as_i64()).unwrap_or(0);
                let max = h.param(1).and_then(|v| v.value().as_i64()).unwrap_or(100);
                let val = rand::thread_rng().gen_range(min..=max);
                out.write(&val.to_string())?;
                Ok(())
            },
        ),
    );

    hbs.register_helper(
        "random_float",
        Box::new(
            |h: &handlebars::Helper,
             _: &Handlebars,
             _: &handlebars::Context,
             _: &mut handlebars::RenderContext,
             out: &mut dyn handlebars::Output|
             -> handlebars::HelperResult {
                let min = h.param(0).and_then(|v| v.value().as_f64()).unwrap_or(0.0);
                let max = h.param(1).and_then(|v| v.value().as_f64()).unwrap_or(1.0);
                let val = rand::thread_rng().gen_range(min..=max);
                out.write(&format!("{:.2}", val))?;
                Ok(())
            },
        ),
    );

    hbs.register_helper(
        "random_bool",
        Box::new(
            |_: &handlebars::Helper,
             _: &Handlebars,
             _: &handlebars::Context,
             _: &mut handlebars::RenderContext,
             out: &mut dyn handlebars::Output|
             -> handlebars::HelperResult {
                let val = rand::thread_rng().gen_bool(0.5);
                out.write(if val { "true" } else { "false" })?;
                Ok(())
            },
        ),
    );

    hbs.register_helper(
        "seq_pulse",
        Box::new(seq_pulse_helper),
    );

    hbs.register_helper(
        "seq_inv_pulse",
        Box::new(seq_inv_pulse_helper),
    );

    hbs.register_helper(
        "seq_after",
        Box::new(seq_after_helper),
    );

    hbs.register_helper(
        "seq_pulse_rand",
        Box::new(seq_pulse_rand_helper),
    );

    hbs.register_helper(
        "seq_inv_pulse_rand",
        Box::new(seq_inv_pulse_rand_helper),
    );

    hbs
}

fn seq_from_context(ctx: &handlebars::Context) -> f64 {
    ctx.data()
        .get("seq")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as f64
}

fn device_index_from_context(ctx: &handlebars::Context) -> u64 {
    ctx.data()
        .get("device")
        .and_then(|d| d.get("index"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0)
}

/// Returns a per-device random peak position, chosen once and cached for the
/// whole run so a device's triangle profile stays stable across its messages.
///
/// Keyed by device index only: every `*_rand` helper for the same device shares
/// this single peak, so active/reactive load and the cos φ dip line up in time.
fn random_peak_for_device(idx: u64, peak_min: f64, peak_max: f64) -> f64 {
    static PEAKS: OnceLock<Mutex<HashMap<u64, f64>>> = OnceLock::new();
    let peaks = PEAKS.get_or_init(|| Mutex::new(HashMap::new()));
    let mut map = peaks.lock().unwrap();
    *map.entry(idx).or_insert_with(|| {
        if peak_max > peak_min {
            rand::thread_rng().gen_range(peak_min..=peak_max)
        } else {
            peak_min
        }
    })
}

/// Triangle pulse: `min` at seq=0 and seq=total_steps, `max` at seq=peak_at.
fn triangle_t(seq: f64, peak_at: f64, total_steps: f64) -> f64 {
    if total_steps <= 0.0 || peak_at <= 0.0 || peak_at >= total_steps {
        return 0.0;
    }
    if seq <= peak_at {
        seq / peak_at
    } else {
        (total_steps - seq) / (total_steps - peak_at)
    }
}

fn seq_pulse_helper(
    h: &handlebars::Helper,
    _: &Handlebars,
    ctx: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let min = h.param(0).and_then(|v| v.value().as_f64()).unwrap_or(0.0);
    let max = h.param(1).and_then(|v| v.value().as_f64()).unwrap_or(1.0);
    let peak_at = h.param(2).and_then(|v| v.value().as_f64()).unwrap_or(1.0);
    let total_steps = h.param(3).and_then(|v| v.value().as_f64()).unwrap_or(1.0);
    let seq = seq_from_context(ctx);
    let t = triangle_t(seq, peak_at, total_steps);
    let val = min + (max - min) * t;
    out.write(&format!("{:.2}", val))?;
    Ok(())
}

/// Inverted triangle pulse: `max` at edges, `min` at peak (e.g. power factor dip mid-month).
fn seq_inv_pulse_helper(
    h: &handlebars::Helper,
    _: &Handlebars,
    ctx: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let min = h.param(0).and_then(|v| v.value().as_f64()).unwrap_or(0.0);
    let max = h.param(1).and_then(|v| v.value().as_f64()).unwrap_or(1.0);
    let peak_at = h.param(2).and_then(|v| v.value().as_f64()).unwrap_or(1.0);
    let total_steps = h.param(3).and_then(|v| v.value().as_f64()).unwrap_or(1.0);
    let seq = seq_from_context(ctx);
    let t = triangle_t(seq, peak_at, total_steps);
    let val = max - (max - min) * t;
    out.write(&format!("{:.4}", val))?;
    Ok(())
}

/// Like `seq_pulse`, but the peak position is drawn randomly (once per device,
/// per run) from `[peak_min, peak_max]`.
/// Params: `min max peak_min peak_max total_steps`.
fn seq_pulse_rand_helper(
    h: &handlebars::Helper,
    _: &Handlebars,
    ctx: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let min = h.param(0).and_then(|v| v.value().as_f64()).unwrap_or(0.0);
    let max = h.param(1).and_then(|v| v.value().as_f64()).unwrap_or(1.0);
    let peak_min = h.param(2).and_then(|v| v.value().as_f64()).unwrap_or(1.0);
    let peak_max = h.param(3).and_then(|v| v.value().as_f64()).unwrap_or(1.0);
    let total_steps = h.param(4).and_then(|v| v.value().as_f64()).unwrap_or(1.0);
    let idx = device_index_from_context(ctx);
    let seq = seq_from_context(ctx);
    let peak_at = random_peak_for_device(idx, peak_min, peak_max);
    let t = triangle_t(seq, peak_at, total_steps);
    let val = min + (max - min) * t;
    out.write(&format!("{:.2}", val))?;
    Ok(())
}

/// Like `seq_inv_pulse`, but the dip position is drawn randomly (once per device,
/// per run) from `[peak_min, peak_max]`. Shares the same per-device peak as
/// `seq_pulse_rand`, so the cos φ dip aligns with the load peak.
/// Params: `min max peak_min peak_max total_steps`.
fn seq_inv_pulse_rand_helper(
    h: &handlebars::Helper,
    _: &Handlebars,
    ctx: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let min = h.param(0).and_then(|v| v.value().as_f64()).unwrap_or(0.0);
    let max = h.param(1).and_then(|v| v.value().as_f64()).unwrap_or(1.0);
    let peak_min = h.param(2).and_then(|v| v.value().as_f64()).unwrap_or(1.0);
    let peak_max = h.param(3).and_then(|v| v.value().as_f64()).unwrap_or(1.0);
    let total_steps = h.param(4).and_then(|v| v.value().as_f64()).unwrap_or(1.0);
    let idx = device_index_from_context(ctx);
    let seq = seq_from_context(ctx);
    let peak_at = random_peak_for_device(idx, peak_min, peak_max);
    let t = triangle_t(seq, peak_at, total_steps);
    let val = max - (max - min) * t;
    out.write(&format!("{:.4}", val))?;
    Ok(())
}

/// Delayed linear ramp: max(0, seq - offset) * scale (e.g. compensation bank cumulative).
fn seq_after_helper(
    h: &handlebars::Helper,
    _: &Handlebars,
    ctx: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let offset = h.param(0).and_then(|v| v.value().as_f64()).unwrap_or(0.0);
    let scale = h.param(1).and_then(|v| v.value().as_f64()).unwrap_or(1.0);
    let seq = seq_from_context(ctx);
    let val = (seq - offset).max(0.0) * scale;
    out.write(&format!("{:.2}", val))?;
    Ok(())
}

/// Render a Handlebars template string with a device context and sequence number.
///
/// `ts` is the simulated timestamp for this message and is exposed to the
/// template as `{{ts}}` (RFC3339). `{{now_utc}}` still returns real wall-clock
/// time regardless of the simulated clock.
pub fn render_template(
    hbs: &Handlebars,
    template_name: &str,
    device: &DeviceContext,
    seq: usize,
    ts: DateTime<Utc>,
) -> Result<String> {
    let data = json!({
        "device_id": device.device_id,
        "device": {
            "id": device.device_id,
            "index": device.index,
        },
        "seq": seq,
        "ts": ts.to_rfc3339(),
    });
    hbs.render(template_name, &data)
        .map_err(|e| MerError::Template(e.to_string()))
}

/// Register a template string in the registry.
pub fn register_template(hbs: &mut Handlebars, name: &str, template: &str) -> Result<()> {
    hbs.register_template_string(name, template)
        .map_err(|e| MerError::Template(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::context::DeviceContext;

    fn make_device(index: usize) -> DeviceContext {
        DeviceContext::new(index, "device")
    }

    fn render(template: &str, device: &DeviceContext, seq: usize) -> String {
        let mut hbs = build_registry();
        register_template(&mut hbs, "t", template).unwrap();
        render_template(&hbs, "t", device, seq, Utc::now()).unwrap()
    }

    #[test]
    fn test_device_id_helper() {
        let device = make_device(5);
        let out = render(r#"{"id":"{{device_id}}"}"#, &device, 0);
        assert!(out.contains("device-0005"), "got: {}", out);
    }

    #[test]
    fn test_device_index_helper() {
        let device = make_device(7);
        let out = render(r#"{"idx":{{device.index}}}"#, &device, 0);
        assert!(out.contains("7"), "got: {}", out);
    }

    #[test]
    fn test_seq_helper() {
        let device = make_device(0);
        let out = render(r#"{"seq":{{seq}}}"#, &device, 42);
        assert!(out.contains("42"), "got: {}", out);
    }

    #[test]
    fn test_ts_variable_uses_simulated_time() {
        let device = make_device(0);
        let mut hbs = build_registry();
        register_template(&mut hbs, "t", r#"{"ts":"{{ts}}"}"#).unwrap();
        let ts = DateTime::parse_from_rfc3339("2026-01-01T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        let out = render_template(&hbs, "t", &device, 0, ts).unwrap();
        assert!(out.contains("2026-01-01T00:00:00+00:00"), "got: {}", out);
    }

    #[test]
    fn test_now_utc_helper_produces_timestamp() {
        let device = make_device(0);
        let out = render(r#"{"ts":"{{now_utc}}"}"#, &device, 0);
        assert!(
            out.contains('T'),
            "expected RFC3339 timestamp, got: {}",
            out
        );
        assert!(
            out.contains('+') || out.contains('Z'),
            "expected timezone, got: {}",
            out
        );
    }

    #[test]
    fn test_random_int_helper_in_range() {
        let device = make_device(0);
        for _ in 0..30 {
            let out = render(r#"{{random_int 10 20}}"#, &device, 0);
            let val: i64 = out.trim().parse().unwrap();
            assert!((10..=20).contains(&val), "out of range: {}", val);
        }
    }

    #[test]
    fn test_random_float_helper_in_range() {
        let device = make_device(0);
        for _ in 0..30 {
            let out = render(r#"{{random_float 1.0 5.0}}"#, &device, 0);
            let val: f64 = out.trim().parse().unwrap();
            assert!((1.0..=5.0).contains(&val), "out of range: {}", val);
        }
    }

    #[test]
    fn test_random_bool_helper_is_bool() {
        let device = make_device(0);
        let mut saw_true = false;
        let mut saw_false = false;
        for _ in 0..50 {
            let out = render(r#"{{random_bool}}"#, &device, 0);
            let s = out.trim();
            assert!(s == "true" || s == "false", "unexpected: {}", s);
            if s == "true" {
                saw_true = true;
            }
            if s == "false" {
                saw_false = true;
            }
        }
        assert!(
            saw_true && saw_false,
            "random_bool should produce both values"
        );
    }

    #[test]
    fn test_invalid_template_returns_error() {
        let mut hbs = build_registry();
        let result = register_template(&mut hbs, "bad", "{{#if}}");
        assert!(result.is_err());
    }

    #[test]
    fn test_seq_pulse_peaks_mid_sequence() {
        let device = make_device(0);
        let out = render(r#"{{seq_pulse 10.0 100.0 50 100}}"#, &device, 50);
        let val: f64 = out.trim().parse().unwrap();
        assert!((val - 100.0).abs() < 0.01, "expected peak 100, got {}", val);

        let low = render(r#"{{seq_pulse 10.0 100.0 50 100}}"#, &device, 0);
        let low_val: f64 = low.trim().parse().unwrap();
        assert!((low_val - 10.0).abs() < 0.01, "expected edge 10, got {}", low_val);
    }

    #[test]
    fn test_seq_inv_pulse_dips_mid_sequence() {
        let device = make_device(0);
        let out = render(r#"{{seq_inv_pulse 0.74 0.96 50 100}}"#, &device, 50);
        let val: f64 = out.trim().parse().unwrap();
        assert!((val - 0.74).abs() < 0.0001, "expected dip 0.74, got {}", val);

        let high = render(r#"{{seq_inv_pulse 0.74 0.96 50 100}}"#, &device, 0);
        let high_val: f64 = high.trim().parse().unwrap();
        assert!(
            (high_val - 0.96).abs() < 0.0001,
            "expected edge 0.96, got {}",
            high_val
        );
    }

    #[test]
    fn test_seq_after_starts_after_offset() {
        let device = make_device(0);
        let before = render(r#"{{seq_after 100 0.1}}"#, &device, 50);
        assert_eq!(before.trim(), "0.00");

        let after = render(r#"{{seq_after 100 0.1}}"#, &device, 150);
        let val: f64 = after.trim().parse().unwrap();
        assert!((val - 5.0).abs() < 0.01, "expected 5.0, got {}", val);
    }

    // Unique device indices below: the random peak is cached process-wide by
    // device index, so each test uses its own index to stay independent.

    #[test]
    fn test_seq_pulse_rand_peak_in_window() {
        let device = make_device(5001);
        let tmpl = r#"{{seq_pulse_rand 10.0 100.0 40 60 100}}"#;
        let mut best_seq = 0usize;
        let mut best_val = f64::MIN;
        for seq in 0..=100 {
            let v: f64 = render(tmpl, &device, seq).trim().parse().unwrap();
            if v > best_val {
                best_val = v;
                best_seq = seq;
            }
        }
        assert!(
            (39..=61).contains(&best_seq),
            "peak seq {} outside random window [40,60]",
            best_seq
        );
        assert!(best_val > 95.0, "peak value too low: {}", best_val);
    }

    #[test]
    fn test_seq_pulse_rand_is_stable_across_calls() {
        let device = make_device(5002);
        let tmpl = r#"{{seq_pulse_rand 0.0 50.0 30 70 200}}"#;
        let a = render(tmpl, &device, 100);
        let b = render(tmpl, &device, 100);
        assert_eq!(a, b, "random peak must stay stable within a run");
    }

    #[test]
    fn test_seq_pulse_rand_edge_is_min() {
        let device = make_device(5003);
        let out = render(r#"{{seq_pulse_rand 7.0 99.0 40 60 100}}"#, &device, 0);
        let v: f64 = out.trim().parse().unwrap();
        assert!((v - 7.0).abs() < 0.01, "edge should be min, got {}", v);
    }

    #[test]
    fn test_seq_inv_pulse_rand_dips_at_load_peak() {
        let device = make_device(5004);
        let pulse = r#"{{seq_pulse_rand 10.0 100.0 40 60 100}}"#;
        let inv = r#"{{seq_inv_pulse_rand 0.80 0.99 40 60 100}}"#;
        let mut best_seq = 0usize;
        let mut best = f64::MIN;
        for seq in 0..=100 {
            let v: f64 = render(pulse, &device, seq).trim().parse().unwrap();
            if v > best {
                best = v;
                best_seq = seq;
            }
        }
        let inv_val: f64 = render(inv, &device, best_seq).trim().parse().unwrap();
        assert!(
            inv_val < 0.82,
            "cos φ should dip where load peaks (shared peak), got {}",
            inv_val
        );
    }
}
