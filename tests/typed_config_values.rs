mod support;

use rusty_kode::{ConfigValue, ConfigValueError, resolve_config_value};
use support::typed_config_evidence_context;

#[test]
fn integer_defaults_resolve_only_strict_signed_i64_text() {
    let context = typed_config_evidence_context();
    let valid_examples = [
        ("0", 0),
        ("+0", 0),
        ("-0", 0),
        ("42", 42),
        ("+42", 42),
        ("-42", -42),
        ("9223372036854775807", i64::MAX),
        ("-9223372036854775808", i64::MIN),
    ];

    for (configured_value, expected) in valid_examples {
        let actual = resolve_config_value(
            "opaque-valid-fixture",
            ConfigValue::Integer(7),
            Some(configured_value),
        );

        assert_eq!(
            actual,
            Ok(ConfigValue::Integer(expected)),
            "{context}, configured_value={configured_value:?}"
        );
    }

    let invalid_examples = [
        " 42",
        "42 ",
        "9_000",
        "١٢",
        "",
        "+",
        "-",
        "42x",
        "9223372036854775808",
        "-9223372036854775809",
    ];

    for configured_value in invalid_examples {
        let actual = resolve_config_value(
            "opaque-invalid-fixture",
            ConfigValue::Integer(7),
            Some(configured_value),
        );

        assert_eq!(
            actual,
            Err(ConfigValueError::InvalidInteger {
                key: "opaque-invalid-fixture".to_owned(),
                value: configured_value.to_owned(),
            }),
            "{context}, configured_value={configured_value:?}"
        );
    }
}

#[test]
fn boolean_defaults_resolve_only_the_pinned_untrimmed_tokens() {
    let context = typed_config_evidence_context();
    let valid_examples = [
        ("1", true),
        ("yes", true),
        ("TRUE", true),
        ("oN", true),
        ("0", false),
        ("NO", false),
        ("False", false),
        ("off", false),
    ];

    for (configured_value, expected) in valid_examples {
        let actual = resolve_config_value(
            "opaque-valid-fixture",
            ConfigValue::Boolean(!expected),
            Some(configured_value),
        );

        assert_eq!(
            actual,
            Ok(ConfigValue::Boolean(expected)),
            "{context}, configured_value={configured_value:?}"
        );
    }

    let invalid_examples = ["", " true", "true ", "enabled", "2"];

    for configured_value in invalid_examples {
        let actual = resolve_config_value(
            "opaque-invalid-fixture",
            ConfigValue::Boolean(false),
            Some(configured_value),
        );

        assert_eq!(
            actual,
            Err(ConfigValueError::InvalidBoolean {
                key: "opaque-invalid-fixture".to_owned(),
                value: configured_value.to_owned(),
            }),
            "{context}, configured_value={configured_value:?}"
        );
    }
}
