//! Code generation for the Rosetta i18n library.
//!
//! # Usage
//! Code generation works within [build script]. You only need to configure source files and
//! the fallback language. Se [Getting started] in the GitHub repository for more information.
//!
//! ```ignore
//! fn main() {
//!     rosetta_build::config()
//!         .source("fr", "locales/fr.json")
//!         .source("en", "locales/en.json")
//!         .fallback("en")
//!         .generate();
//! }
//! ```
//!
//! [build script]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
//! [Getting started]: https://github.com/baptiste0928/rosetta

pub mod error;

mod config;
mod gen;
mod parser;

use std::{
    collections::HashMap,
    fmt::{self, Display},
    path::PathBuf,
    str::FromStr,
};

use crate::error::{BuildError, ConfigError};

/// Helper function that return an default [`RosettaBuilder`].
pub fn config() -> RosettaBuilder {
    RosettaBuilder::default()
}

/// Builder used to configure Rosetta code generation.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RosettaBuilder {
    files: HashMap<String, PathBuf>,
    fallback: Option<String>,
    name: Option<String>,
    output: Option<PathBuf>,
}

impl RosettaBuilder {
    /// Register a new translation source
    pub fn source(mut self, lang: impl Into<String>, path: impl Into<String>) -> Self {
        self.files.insert(lang.into(), PathBuf::from(path.into()));
        self
    }

    /// Register the fallback locale
    pub fn fallback(mut self, lang: impl Into<String>) -> Self {
        self.fallback = Some(lang.into());
        self
    }

    /// Define a custom name for the output type
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Change the default output of generated files
    pub fn output(mut self, path: impl Into<PathBuf>) -> Self {
        self.output = Some(path.into());
        self
    }

    /// Generate locale files and write them to the output location
    pub fn generate(self) -> Result<(), BuildError> {
        self.build()?.generate()?;
        Ok(())
    }

    /// Validate configuration and build a [`RosettaConfig`]
    fn build(self) -> Result<config::RosettaConfig, ConfigError> {
        let mut files: HashMap<LanguageId, PathBuf> = self
            .files
            .into_iter()
            .map(|(lang, path)| {
                let lang = lang.parse::<LanguageId>()?;
                Ok((lang, path))
            })
            .collect::<Result<_, _>>()?;

        if files.is_empty() {
            return Err(ConfigError::MissingSource);
        }

        let fallback = match self.fallback {
            Some(lang) => {
                let lang = lang.parse::<LanguageId>()?;

                match files.remove_entry(&lang) {
                    Some(entry) => entry,
                    None => return Err(ConfigError::InvalidFallback),
                }
            }
            None => return Err(ConfigError::MissingFallback),
        };

        Ok(config::RosettaConfig {
            fallback,
            others: files,
            name: self.name.unwrap_or_else(|| "Lang".to_string()),
            output: self.output,
        })
    }
}

/// ISO 639-1 language identifier.
///
/// Language identifier can be validated using the [`FromStr`] trait.
/// It only checks if the string *looks like* a language identifier (2 character alphanumeric ascii string).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct LanguageId(String);

impl LanguageId {
    fn value(&self) -> &str {
        &self.0
    }
}

impl FromStr for LanguageId {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let valid_length = s.len() == 2;
        let ascii_alphabetic = s.chars().all(|c| c.is_ascii_alphabetic());

        if valid_length && ascii_alphabetic {
            Ok(Self(s.to_ascii_lowercase()))
        } else {
            Err(ConfigError::InvalidLanguage(s.into()))
        }
    }
}

impl Display for LanguageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
