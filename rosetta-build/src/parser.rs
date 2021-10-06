//! Translations files parsing
//!
//! Files are parsed as [TranslationData] from a provided [JsonValue].
//! Parsed keys are represented as [TranslationKey].

use std::collections::HashMap;

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TranslationKey {
    /// Simple string key, without any interpolation or plurals
    Simple {
        /// The key value for the fallback language
        fallback: String,
        /// Key values for other languages
        others: HashMap<LanguageId, String>,
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
        language: LanguageId,
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
}

impl ParsedKey {
    /// Parse a JSON [`Value`] as a key
    fn parse(key: &str, value: JsonValue) -> Result<Self, ParseError> {
        match value {
            JsonValue::String(value) => Ok(ParsedKey::Simple(value)),
            _ => Err(ParseError::InvalidValue { key: key.into() }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{TranslationData, TranslationKey};
    use crate::{error::ParseError, LanguageId};

    use maplit::hashmap;
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

        let expected = TranslationKey::Simple {
            fallback: "Hello world!".to_string(),
            others: hashmap! {
                LanguageId("fr".into()) => "Bonjour le monde !".to_string()
            },
        };

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
}
