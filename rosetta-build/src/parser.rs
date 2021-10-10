//! Translations files parsing
//!
//! Files are parsed as [TranslationData] from a provided [JsonValue].
//! Parsed keys are represented as [TranslationKey].

use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;
use tinyjson::JsonValue;

use crate::{error::ParseError, LanguageId};

/// Data structure containing all translation keys
///
/// This struct should be initialized with the fallback language,
/// then keys will be populated with other languages using the [`parse_file`] method.
///
/// [`parse_file`]: Self::parse_file
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TranslationData {
    /// Parsed translation keys
    pub(crate) keys: HashMap<String, TranslationKey>,
}

impl TranslationData {
    /// Initialize a [`TranslationData`] instance from the fallback language
    pub(crate) fn from_fallback(file: JsonValue) -> Result<Self, ParseError> {
        let parsed = ParsedFile::parse(file)?;
        let keys = parsed
            .keys
            .into_iter()
            .map(|(key, value)| (key, TranslationKey::from_parsed(value)))
            .collect();

        Ok(Self { keys })
    }

    /// Parse a language file and insert its content into the current [`TranslationData`]
    pub(crate) fn parse_file(
        &mut self,
        language: LanguageId,
        file: JsonValue,
    ) -> Result<(), ParseError> {
        let parsed = ParsedFile::parse(file)?;

        for (key, parsed) in parsed.keys {
            match self.keys.get_mut(&key) {
                Some(translation_key) => {
                    let data = ParsedKeyData {
                        language: language.clone(),
                        key: &key,
                        parsed,
                    };
                    translation_key.insert_parsed(data)?
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TranslationKey {
    Simple(SimpleKey),
    Formatted(FormattedKey),
}

impl TranslationKey {
    /// Initialize a new [TranslationKey] from a [`ParsedKey`]
    fn from_parsed(parsed: ParsedKey) -> Self {
        match parsed {
            ParsedKey::Simple(value) => TranslationKey::Simple(SimpleKey {
                fallback: value,
                others: HashMap::new(),
            }),
            ParsedKey::Formatted { value, parameters } => TranslationKey::Formatted(FormattedKey {
                fallback: value,
                others: HashMap::new(),
                parameters,
            }),
        }
    }

    /// Inserts a new raw [`ParsedKey`] in this [`TranslationKey`]
    fn insert_parsed(&mut self, data: ParsedKeyData) -> Result<(), ParseError> {
        match self {
            TranslationKey::Simple(inner) => inner.insert_parsed(data),
            TranslationKey::Formatted(inner) => inner.insert_parsed(data),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Simple string key, without any formatting or plurals
pub(crate) struct SimpleKey {
    /// The key value for the fallback language
    pub(crate) fallback: String,
    /// Key values for other languages
    pub(crate) others: HashMap<LanguageId, String>,
}

impl SimpleKey {
    /// Inserts a new raw [`ParsedKey`] in this [`SimpleKey`]
    fn insert_parsed(&mut self, data: ParsedKeyData) -> Result<(), ParseError> {
        match data.parsed {
            ParsedKey::Simple(value) => self.others.insert(data.language, value),
            _ => {
                return Err(ParseError::InvalidType {
                    key: data.key.into(),
                    expected: "string",
                })
            }
        };

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Simple string key with formatting
pub(crate) struct FormattedKey {
    /// The key value for the fallback language
    pub(crate) fallback: String,
    /// Key values for other languages
    pub(crate) others: HashMap<LanguageId, String>,
    /// List of parameters in the value
    pub(crate) parameters: HashSet<String>,
}

impl FormattedKey {
    /// Inserts a new [`ParsedKey`] in this [`SimpleKey`]
    fn insert_parsed(&mut self, data: ParsedKeyData) -> Result<(), ParseError> {
        let (value, parameters) = match data.parsed {
            ParsedKey::Formatted { value, parameters } => (value, parameters),
            _ => {
                return Err(ParseError::InvalidType {
                    key: data.key.into(),
                    expected: "formatted string",
                })
            }
        };

        if parameters == self.parameters {
            self.others.insert(data.language, value);
            Ok(())
        } else {
            let missing: Vec<_> = self.parameters.difference(&parameters).cloned().collect();
            let unknown: Vec<_> = parameters.difference(&self.parameters).cloned().collect();

            Err(ParseError::InvalidParameters {
                key: data.key.into(),
                missing,
                unknown,
            })
        }
    }
}

/// Raw representation of a parsed file
#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedFile {
    keys: HashMap<String, ParsedKey>,
}

impl ParsedFile {
    /// Parse a JSON [`JsonValue`] as a translations file
    fn parse(file: JsonValue) -> Result<Self, ParseError> {
        let input = match file {
            JsonValue::Object(map) => map,
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
#[derive(Debug, Clone, PartialEq, Eq)]
enum ParsedKey {
    /// Simple string key
    Simple(String),
    /// String key with formatted values
    ///
    /// Example : `Hello {name}!`
    Formatted {
        /// The raw key value
        value: String,
        /// List of parameters in the value
        parameters: HashSet<String>,
    },
}

impl ParsedKey {
    /// Parse a JSON [`Value`] as a key
    fn parse(key: &str, value: JsonValue) -> Result<Self, ParseError> {
        match value {
            JsonValue::String(value) => Ok(Self::parse_string(value)),
            _ => Err(ParseError::InvalidValue { key: key.into() }),
        }
    }

    fn parse_string(value: String) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\{([a-z_]+)\}").unwrap();
        }

        let matches: HashSet<_> = RE
            .captures_iter(&value)
            .map(|capture| capture[1].to_string())
            .collect();

        if matches.is_empty() {
            Self::Simple(value)
        } else {
            Self::Formatted {
                value,
                parameters: matches,
            }
        }
    }
}

/// Data associated with a parsed key.
///
/// Used in [`TranslationKey::insert_parsed`].
#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedKeyData<'a> {
    language: LanguageId,
    key: &'a str,
    parsed: ParsedKey,
}

#[cfg(test)]
mod tests {
    use super::{TranslationData, TranslationKey};
    use crate::{
        error::ParseError,
        parser::{FormattedKey, SimpleKey},
        LanguageId,
    };

    use maplit::{hashmap, hashset};
    use tinyjson::JsonValue;

    macro_rules! json {
        ($value:tt) => {
            stringify!($value).parse::<JsonValue>().unwrap()
        };
    }

    #[test]
    fn parse_simple() -> Result<(), Box<dyn std::error::Error>> {
        let en = json!({ "hello": "Hello world!" });
        let fr = json!({ "hello": "Bonjour le monde !" });

        let mut parsed = TranslationData::from_fallback(en)?;
        parsed.parse_file(LanguageId("fr".into()), fr)?;

        assert_eq!(parsed.keys.len(), 1);
        assert!(parsed.keys.get("hello").is_some());

        let expected = TranslationKey::Simple(SimpleKey {
            fallback: "Hello world!".to_string(),
            others: hashmap! {
                LanguageId("fr".into()) => "Bonjour le monde !".to_string()
            },
        });

        assert_eq!(parsed.keys.get("hello").unwrap(), &expected);

        Ok(())
    }

    #[test]
    fn parse_formatted() -> Result<(), Box<dyn std::error::Error>> {
        let en = json!({ "hello": "Hello {name}!" });
        let fr = json!({ "hello": "Bonjour {name} !" });

        let mut parsed = TranslationData::from_fallback(en)?;
        parsed.parse_file(LanguageId("fr".into()), fr)?;

        assert_eq!(parsed.keys.len(), 1);
        assert!(parsed.keys.get("hello").is_some());

        let expected = TranslationKey::Formatted(FormattedKey {
            fallback: "Hello {name}!".to_string(),
            others: hashmap! {
                LanguageId("fr".into()) => "Bonjour {name} !".to_string()
            },
            parameters: hashset! { "name".to_string() },
        });

        assert_eq!(parsed.keys.get("hello").unwrap(), &expected);

        Ok(())
    }

    #[test]
    fn parse_invalid_root() {
        let file = json!("invalid");
        let parsed = TranslationData::from_fallback(file);
        assert_eq!(parsed, Err(ParseError::InvalidRoot));
    }

    #[test]
    fn parse_invalid_value() {
        let file = json!({ "hello": ["Hello world!"] });
        let parsed = TranslationData::from_fallback(file);
        assert_eq!(
            parsed,
            Err(ParseError::InvalidValue {
                key: "hello".to_string()
            })
        );
    }

    #[test]
    fn parse_invalid_parameter() {
        let en = json!({ "hello": "Hello {name}!" });
        let fr = json!({ "hello": "Bonjour {surname} !" });

        let mut parsed = TranslationData::from_fallback(en).unwrap();
        let result = parsed.parse_file(LanguageId("fr".into()), fr);

        let expected = ParseError::InvalidParameters {
            key: "hello".to_string(),
            missing: vec!["name".to_string()],
            unknown: vec!["surname".to_string()],
        };
        assert_eq!(result, Err(expected));
    }
}
