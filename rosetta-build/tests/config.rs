use std::path::PathBuf;

use icu_locid_macros::langid;
use maplit::hashmap;
use rosetta_build::{ConfigError, RosettaConfig};

#[test]
fn config_simple() -> Result<(), Box<dyn std::error::Error>> {
    let config = RosettaConfig::builder()
        .source("en", "translations/en.json")
        .source("fr", "translations/fr.json")
        .fallback("en")
        .build()?;

    let expected = RosettaConfig {
        fallback: (langid!("en"), PathBuf::from("translations/en.json")),
        others: hashmap! { langid!("fr") => PathBuf::from("translations/fr.json") },
        output: None,
    };

    assert_eq!(config, expected);

    Ok(())
}

#[test]
fn config_missing_source() {
    let config = RosettaConfig::builder().build();
    assert_eq!(config, Err(ConfigError::MissingSource));
}

#[test]
fn config_invalid_language() {
    let config = RosettaConfig::builder()
        .source("en", "translations/en.json")
        .source("invalid", "translations/fr.json")
        .fallback("en")
        .build();

    assert_eq!(
        config,
        Err(ConfigError::InvalidLanguage("invalid".to_string()))
    );
}

#[test]
fn config_missing_fallback() {
    let config = RosettaConfig::builder()
        .source("en", "translations/en.json")
        .source("fr", "translations/fr.json")
        .build();

    assert_eq!(config, Err(ConfigError::MissingFallback));
}

#[test]
fn config_invalid_fallback() {
    let config = RosettaConfig::builder()
        .source("en", "translations/en.json")
        .source("fr", "translations/fr.json")
        .fallback("de")
        .build();

    assert_eq!(config, Err(ConfigError::InvalidFallback));
}
