use crate::config::model::Config;
use crate::error::{MerError, Result};
use regex::Regex;
use std::path::Path;

pub fn load_config(path: &Path) -> Result<Config> {
    let raw = read_text_file(path)?;
    let expanded = expand_env_vars(&raw)?;
    let config: Config = serde_yaml::from_str(&expanded)?;
    Ok(config)
}

/// Read a config file as text, accepting UTF-8 and UTF-16 (with BOM).
/// Windows PowerShell redirection (`> file`) writes UTF-16 LE by default.
pub fn read_text_file(path: &Path) -> Result<String> {
    let bytes = std::fs::read(path)?;
    decode_text_bytes(&bytes)
}

fn decode_text_bytes(bytes: &[u8]) -> Result<String> {
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        String::from_utf8(bytes[3..].to_vec()).map_err(invalid_utf8_error)
    } else if bytes.starts_with(&[0xFF, 0xFE]) {
        decode_utf16_le(&bytes[2..])
    } else if bytes.starts_with(&[0xFE, 0xFF]) {
        decode_utf16_be(&bytes[2..])
    } else {
        String::from_utf8(bytes.to_vec()).map_err(invalid_utf8_error)
    }
}

fn decode_utf16_le(bytes: &[u8]) -> Result<String> {
    if bytes.len() % 2 != 0 {
        return Err(MerError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "invalid UTF-16 LE data: odd byte length",
        )));
    }
    let units: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();
    String::from_utf16(&units).map_err(|e| {
        MerError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("invalid UTF-16 LE data: {e}"),
        ))
    })
}

fn decode_utf16_be(bytes: &[u8]) -> Result<String> {
    if bytes.len() % 2 != 0 {
        return Err(MerError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "invalid UTF-16 BE data: odd byte length",
        )));
    }
    let units: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
        .collect();
    String::from_utf16(&units).map_err(|e| {
        MerError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("invalid UTF-16 BE data: {e}"),
        ))
    })
}

fn invalid_utf8_error(err: std::string::FromUtf8Error) -> MerError {
    MerError::Io(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        format!("stream did not contain valid UTF-8: {err}"),
    ))
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
    fn test_read_text_file_utf16_le_bom() {
        let path = std::env::temp_dir().join(format!("mer-utf16-{}.yaml", uuid::Uuid::new_v4()));
        let content = "protocol: mqtt\n";
        let mut bytes = vec![0xFF, 0xFE];
        for ch in content.encode_utf16() {
            bytes.extend_from_slice(&ch.to_le_bytes());
        }
        std::fs::write(&path, bytes).unwrap();
        assert_eq!(read_text_file(&path).unwrap(), content);
        let _ = std::fs::remove_file(path);
    }
}
