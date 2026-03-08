use crate::config::model::Config;
use crate::error::{MerError, Result};
use regex::Regex;
use std::path::Path;

pub fn load_config(path: &Path) -> Result<Config> {
    let raw = std::fs::read_to_string(path)?;
    let expanded = expand_env_vars(&raw)?;
    let config: Config = serde_yaml::from_str(&expanded)?;
    Ok(config)
}

/// Expand `${VAR_NAME}` placeholders with environment variable values.
/// Returns an error if a referenced variable is not set.
pub fn expand_env_vars(input: &str) -> Result<String> {
    let re = Regex::new(r"\$\{([A-Za-z_][A-Za-z0-9_]*)\}").expect("valid regex");
    let mut error: Option<MerError> = None;
    let result = re.replace_all(input, |caps: &regex::Captures| {
        let name = &caps[1];
        match std::env::var(name) {
            Ok(val) => val,
            Err(_) => {
                if error.is_none() {
                    error = Some(MerError::MissingEnvVar {
                        name: name.to_string(),
                    });
                }
                String::new()
            }
        }
    });
    if let Some(e) = error {
        return Err(e);
    }
    Ok(result.into_owned())
}
