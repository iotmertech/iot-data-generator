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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_expand_env_vars_no_placeholders() {
        let input = "broker: mqtt://localhost:1883";
        assert_eq!(expand_env_vars(input).unwrap(), input);
    }

    #[test]
    fn test_expand_env_vars_substitutes_value() {
        env::set_var("MER_TEST_BROKER", "mqtt://broker.example.com:1883");
        let input = "broker: ${MER_TEST_BROKER}";
        let result = expand_env_vars(input).unwrap();
        assert_eq!(result, "broker: mqtt://broker.example.com:1883");
        env::remove_var("MER_TEST_BROKER");
    }

    #[test]
    fn test_expand_env_vars_multiple_placeholders() {
        env::set_var("MER_TEST_USER", "alice");
        env::set_var("MER_TEST_PASS", "s3cr3t");
        let input = "username: ${MER_TEST_USER}\npassword: ${MER_TEST_PASS}";
        let result = expand_env_vars(input).unwrap();
        assert_eq!(result, "username: alice\npassword: s3cr3t");
        env::remove_var("MER_TEST_USER");
        env::remove_var("MER_TEST_PASS");
    }

    #[test]
    fn test_expand_env_vars_missing_var_returns_error() {
        env::remove_var("MER_TEST_NONEXISTENT_VAR_XYZ");
        let input = "token: ${MER_TEST_NONEXISTENT_VAR_XYZ}";
        let err = expand_env_vars(input).unwrap_err();
        match err {
            MerError::MissingEnvVar { name } => {
                assert_eq!(name, "MER_TEST_NONEXISTENT_VAR_XYZ");
            }
            other => panic!("Expected MissingEnvVar, got: {:?}", other),
        }
    }

    #[test]
    fn test_expand_env_vars_no_expansion_in_non_placeholder() {
        let input = "topic: devices/plain/topic";
        assert_eq!(expand_env_vars(input).unwrap(), input);
    }
}
