use crate::device::DeviceContext;
use crate::error::{MerError, Result};
use chrono::Utc;
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
                let min = h
                    .param(0)
                    .and_then(|v| v.value().as_i64())
                    .unwrap_or(0);
                let max = h
                    .param(1)
                    .and_then(|v| v.value().as_i64())
                    .unwrap_or(100);
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
                let min = h
                    .param(0)
                    .and_then(|v| v.value().as_f64())
                    .unwrap_or(0.0);
                let max = h
                    .param(1)
                    .and_then(|v| v.value().as_f64())
                    .unwrap_or(1.0);
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
pub fn render_template(
    hbs: &Handlebars,
    template_name: &str,
    device: &DeviceContext,
    seq: usize,
) -> Result<String> {
    let data = json!({
        "device_id": device.device_id,
        "device": {
            "id": device.device_id,
            "index": device.index,
        },
        "seq": seq,
    });
    hbs.render(template_name, &data)
        .map_err(|e| MerError::Template(e.to_string()))
}

/// Register a template string in the registry.
pub fn register_template(hbs: &mut Handlebars, name: &str, template: &str) -> Result<()> {
    hbs.register_template_string(name, template)
        .map_err(|e| MerError::Template(e.to_string()))
}
