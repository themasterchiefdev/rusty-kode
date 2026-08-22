#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConfigValue {
    Integer(i64),
    Boolean(bool),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConfigValueError {
    InvalidInteger { key: String, value: String },
    InvalidBoolean { key: String, value: String },
}

pub fn resolve_config_value(
    key: &str,
    declared_default: ConfigValue,
    configured_value: Option<&str>,
) -> Result<ConfigValue, ConfigValueError> {
    match (declared_default, configured_value) {
        (ConfigValue::Integer(_), Some(value)) => resolve_integer(key, value),
        _ => unimplemented!("later MET-020 slices resolve other default variants"),
    }
}

fn resolve_integer(key: &str, value: &str) -> Result<ConfigValue, ConfigValueError> {
    let digits = value.strip_prefix(['+', '-']).unwrap_or(value).as_bytes();
    let has_strict_integer_grammar = !digits.is_empty() && digits.iter().all(u8::is_ascii_digit);

    if has_strict_integer_grammar && let Ok(integer) = value.parse::<i64>() {
        return Ok(ConfigValue::Integer(integer));
    }

    Err(ConfigValueError::InvalidInteger {
        key: key.to_owned(),
        value: value.to_owned(),
    })
}
