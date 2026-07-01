use crate::device::DeviceContext;
use crate::error::{MerError, Result};
use chrono::{DateTime, Utc};
use handlebars::Handlebars;
use rand::Rng;
use serde_json::json;

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

    hbs
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
}
