use crate::config::model::{Config, PayloadMode};
use crate::device::DeviceContext;
use crate::error::{MerError, Result};
use crate::payload::{profiles, template as tmpl};
use handlebars::Handlebars;
use serde_json::Value;

pub struct PayloadGenerator {
    mode: PayloadMode,
    hbs: Option<Handlebars<'static>>,
}

impl PayloadGenerator {
    pub fn from_config(config: &Config) -> Result<Self> {
        match config.payload.mode {
            PayloadMode::Random => Ok(Self {
                mode: PayloadMode::Random,
                hbs: None,
            }),
            PayloadMode::Template => {
                let mut hbs = tmpl::build_registry();
                let template_str = if let Some(file) = &config.payload.template_file {
                    std::fs::read_to_string(file).map_err(|e| {
                        MerError::Config(format!("Cannot read template file '{}': {}", file, e))
                    })?
                } else if let Some(inline) = &config.payload.template_inline {
                    inline.clone()
                } else {
                    return Err(MerError::Config(
                        "Template mode requires template_file or template_inline".to_string(),
                    ));
                };
                tmpl::register_template(&mut hbs, "payload", &template_str)?;
                Ok(Self {
                    mode: PayloadMode::Template,
                    hbs: Some(hbs),
                })
            }
        }
    }

    pub fn generate(&self, device: &DeviceContext, seq: usize) -> Result<String> {
        match self.mode {
            PayloadMode::Random => {
                let val: Value = profiles::generate_random(device, seq)?;
                Ok(serde_json::to_string(&val)?)
            }
            PayloadMode::Template => {
                let hbs = self.hbs.as_ref().expect("hbs must be set in template mode");
                tmpl::render_template(hbs, "payload", device, seq)
            }
        }
    }

    pub fn generate_preview(&self, device: &DeviceContext, seq: usize) -> Result<String> {
        let raw = self.generate(device, seq)?;
        // Pretty-print if valid JSON
        if let Ok(val) = serde_json::from_str::<Value>(&raw) {
            Ok(serde_json::to_string_pretty(&val)?)
        } else {
            Ok(raw)
        }
    }
}
