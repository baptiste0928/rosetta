//! Helpers to use serialize and deserialize types that implement [`Language`].
//!
//! ## Provided helpers
//! This module provide the [`as_language`] and [`as_language_with_fallback`] serde helpers.
//! These helpers can be used to serialize and deserialize any type that implements [`Language`].
//!
//! The [`as_language`] helper will produce an error when trying to deserialize an unsupported language,
//! whereas the [`as_language_with_fallback`] helper will return the fallback value.
//!
//! ## Example
//! ```rust
//! # use serde::{Serialize, Deserialize};
//! use rosetta_i18n::{
//!     GenericLanguage,
//!     serde_helpers::{as_language, as_language_with_fallback}
//! };
//!
//! #[derive(Serialize, Deserialize)]
//! struct Config {
//!     #[serde(with = "as_language")]
//!     pub language: GenericLanguage,
//!     #[serde(with = "as_language_with_fallback")]
//!     pub language_fallback: GenericLanguage,
//! }
//! ```
//!
//! [`Language`]: crate::Language

use std::borrow::Cow;

use serde::{de, ser};

use crate::LanguageId;

impl ser::Serialize for LanguageId<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> de::Deserialize<'de> for LanguageId<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let value = Cow::deserialize(deserializer)?;

        match Self::validate(&value) {
            Some(language_id) => Ok(language_id),
            None => Err(de::Error::custom(format!(
                "`{}` is not a valid ISO 693-1 language id",
                value
            ))),
        }
    }
}

pub mod as_language {
    //! Serialize and deserialize a type that implements [`Language`].
    //!
    //! ## Example
    //! ```rust
    //! # use serde::{Serialize, Deserialize};
    //! # use rosetta_i18n::serde_helpers::as_language;
    //! #[derive(Serialize, Deserialize)]
    //! struct Config {
    //!     #[serde(with = "as_language")]
    //!     pub language: rosetta_i18n::GenericLanguage
    //! }
    //! ```

    use serde::{de, ser, Deserialize, Serialize};

    use crate::{Language, LanguageId};

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: de::Deserializer<'de>,
        T: Language,
    {
        let language_id = LanguageId::deserialize(deserializer)?;

        match T::from_language_id(&language_id) {
            Some(value) => Ok(value),
            None => Err(de::Error::custom("language `{}` is not supported")),
        }
    }

    pub fn serialize<S, T>(val: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
        T: Language,
    {
        val.language_id().serialize(serializer)
    }
}

pub mod as_language_with_fallback {
    //! Serialize and deserialize a type that implements [`Language`] with fallback value.
    //!
    //! If the language is unsupported, the fallback language is used.
    //!
    //! ## Example
    //! ```rust
    //! # use serde::{Serialize, Deserialize};
    //! # use rosetta_i18n::serde_helpers::as_language_with_fallback;
    //! #[derive(Serialize, Deserialize)]
    //! struct Config {
    //!     #[serde(with = "as_language_with_fallback")]
    //!     pub language: rosetta_i18n::GenericLanguage
    //! }
    //! ```

    use serde::{de, ser, Deserialize, Serialize};

    use crate::{Language, LanguageId};

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: de::Deserializer<'de>,
        T: Language,
    {
        let language_id = LanguageId::deserialize(deserializer)?;

        match T::from_language_id(&language_id) {
            Some(value) => Ok(value),
            None => Ok(T::fallback()),
        }
    }

    pub fn serialize<S, T>(val: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
        T: Language,
    {
        val.language_id().serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_test::{assert_de_tokens, assert_de_tokens_error, assert_tokens, Token};

    use crate::Language;

    use super::{as_language, as_language_with_fallback, LanguageId};

    #[test]
    fn serde_language_id() {
        let lang_id = LanguageId::new("en");
        assert_tokens(&lang_id, &[Token::String("en")]);
    }

    #[test]
    fn serde_invalid_language_id() {
        assert_de_tokens_error::<LanguageId>(
            &[Token::String("invalid")],
            "`invalid` is not a valid ISO 693-1 language id",
        )
    }

    #[test]
    fn serde_as_language() {
        #[derive(Debug, PartialEq, Eq)]
        struct Lang;

        impl Language for Lang {
            fn from_language_id(language_id: &LanguageId) -> Option<Self> {
                match language_id.value() {
                    "en" => Some(Self),
                    _ => None,
                }
            }

            fn language_id(&self) -> LanguageId {
                LanguageId::new("en")
            }

            fn fallback() -> Self {
                unimplemented!()
            }
        }

        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
        struct LanguageStruct {
            #[serde(with = "as_language")]
            lang: Lang,
        }

        let value = LanguageStruct { lang: Lang };

        assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "LanguageStruct",
                    len: 1,
                },
                Token::Str("lang"),
                Token::String("en"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn serde_as_language_fallback() {
        #[derive(Debug, PartialEq, Eq)]
        struct Lang;

        impl Language for Lang {
            fn from_language_id(language_id: &LanguageId) -> Option<Self> {
                match language_id.value() {
                    "en" => Some(Self),
                    _ => None,
                }
            }

            fn language_id(&self) -> LanguageId {
                LanguageId::new("en")
            }

            fn fallback() -> Self {
                Self
            }
        }

        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
        struct LanguageStruct {
            #[serde(with = "as_language_with_fallback")]
            lang: Lang,
        }

        let value = LanguageStruct { lang: Lang };

        assert_de_tokens(
            &value,
            &[
                Token::Struct {
                    name: "LanguageStruct",
                    len: 1,
                },
                Token::Str("lang"),
                Token::String("fr"),
                Token::StructEnd,
            ],
        );
    }
}
