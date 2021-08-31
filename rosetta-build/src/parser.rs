//! Translations files parsing
//!
//! # Usage
//! Parsed translations files are represented as a [TranslationData].
//! This type must be initialized with the fallback language before parsing other languages.
//! Files must be provided as JSON [`Value`].
//!
//! ```
//! use rosetta_build::parser::TranslationData;
//! # use serde_json::json;
//! # use icu_locid_macros::langid;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let en = json!({ "hello": "Hello world!" });
//! let fr = json!({ "hello": "Bonjour le monde !" });
//!
//! let mut parsed = TranslationData::from_fallback(en)?;
//! parsed.parse_file(langid!("fr"), fr)?;
//!
//! assert_eq!(parsed.keys.len(), 1);
//! # Ok(())
//! # }
//! ```
//!
//! Parsed keys are represented as [TranslationKey].

use std::collections::HashMap;

use icu_locid::LanguageIdentifier;
use serde_json::Value;
use thiserror::Error;

/// Data structure containing all translation keys
///
/// This struct should be initialized with the fallback language,
/// then keys will be populated with other languages using the [`parse_file`] method.
///
/// [`parse_file`]: Self::parse_file
#[derive(Debug, PartialEq, Eq)]
pub struct TranslationData {
    /// Parsed translation keys
    pub keys: HashMap<String, TranslationKey>,
}

impl TranslationData {
    /// Initialize a [`TranslationData`] instance from the fallback language
    pub fn from_fallback(file: Value) -> Result<Self, ParseError> {
        let parsed = ParsedFile::parse(file)?;
        let keys = parsed
            .keys
            .into_iter()
            .map(|(key, value)| (key, TranslationKey::from_parsed(value)))
            .collect();

        Ok(Self { keys })
    }

    /// Parse a language file and insert its content into the current [`TranslationData`]
    pub fn parse_file(
        &mut self,
        language: LanguageIdentifier,
        file: Value,
    ) -> Result<(), ParseError> {
        let parsed = ParsedFile::parse(file)?;
        for (key, parsed) in parsed.keys {
            match self.keys.get_mut(&key) {
                Some(translation_key) => {
                    translation_key.insert_parsed(language.clone(), &key, parsed)?
                }
                None => println!(
                    "cargo:warning=Key `{}` exists in {} but not in fallback language",
                    key, language
                ),
            };
        }

        Ok(())
    }
}

/// A parsed translation key
///
/// This enum can be constructed by parsing a translation file with [TranslationData].
#[derive(Debug, PartialEq, Eq)]
pub enum TranslationKey {
    /// Simple string key, without any interpolation or plurals
    Simple {
        /// The key value for the fallback language
        fallback: String,
        /// Key values for other languages
        others: HashMap<LanguageIdentifier, String>,
    },
}

impl TranslationKey {
    /// Initialize a new [TranslationKey] from a [`ParsedKey`]
    fn from_parsed(parsed: ParsedKey) -> Self {
        match parsed {
            ParsedKey::Simple(value) => TranslationKey::Simple {
                fallback: value,
                others: HashMap::new(),
            },
        }
    }

    /// Inserts a new raw [`ParsedKey`] in this [TranslationKey]
    #[allow(unreachable_patterns)]
    fn insert_parsed(
        &mut self,
        language: LanguageIdentifier,
        key: &str,
        parsed: ParsedKey,
    ) -> Result<(), ParseError> {
        match self {
            TranslationKey::Simple {
                fallback: _,
                others,
            } => match parsed {
                ParsedKey::Simple(value) => others.insert(language, value),
                _ => {
                    return Err(ParseError::InvalidType {
                        key: key.into(),
                        expected: "string",
                    })
                }
            },
        };

        Ok(())
    }
}

/// Raw representation of a parsed file
#[derive(Debug, PartialEq, Eq)]
struct ParsedFile {
    keys: HashMap<String, ParsedKey>,
}

impl ParsedFile {
    /// Parse a JSON [`Value`] as a translations file
    fn parse(file: Value) -> Result<Self, ParseError> {
        let input = match file {
            Value::Object(map) => map,
            _ => return Err(ParseError::InvalidRoot),
        };

        let mut keys = HashMap::with_capacity(input.len());
        for (key, value) in input {
            let parsed = ParsedKey::parse(&key, value)?;
            keys.insert(key, parsed);
        }

        Ok(ParsedFile { keys })
    }
}

/// Raw representation of a parsed key
#[derive(Debug, PartialEq, Eq)]
enum ParsedKey {
    /// Simple string key
    Simple(String),
}

impl ParsedKey {
    /// Parse a JSON [`Value`] as a key
    fn parse(key: &str, value: Value) -> Result<Self, ParseError> {
        match value {
            Value::String(value) => Ok(ParsedKey::Simple(value)),
            _ => Err(ParseError::InvalidValue { key: key.into() }),
        }
    }
}

/// Error type returned when a parsing error occurs
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseError {
    /// File root is not a JSON object
    #[error("file root must be a json object")]
    InvalidRoot,
    /// Invalid key type (raw parsing)
    #[error("`{key}` is an invalid type")]
    InvalidValue { key: String },
    /// Invalid key type (doesn't match previous parsed keys)
    #[error("`{key}` doesn't match previous parsed key (expected {expected})")]
    InvalidType { key: String, expected: &'static str },
}
