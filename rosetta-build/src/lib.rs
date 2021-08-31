//! Code generation for the Rosetta i18n library
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
//! # Advanced usage
//! This crate is not intended to be used outside a build script, but if you have special needs,
//! the parser and codegen modules are publicly exposed and can be used separately.
//! Refer to their documentation for more information.
//!
//! [build script]: https://doc.rust-lang.org/cargo/reference/build-scripts.html

pub mod parser;

use std::collections::HashMap;
use std::path::PathBuf;

use icu_locid::LanguageIdentifier;
use thiserror::Error;

/// Helper function that return an default [`RosettaBuilder`]
pub fn config() -> RosettaBuilder {
    RosettaBuilder::default()
}

/// Builder used to configure Rosetta code generation.
///
/// Please see [Getting started] on the GitHub repository for usage instructions.
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
    pub fn generate(self) {
        let _ = self.build();
    }

    /// Validate configuration and build a [`RosettaConfig`]
    ///
    /// You probably don't need to call this explicitly, use [`generate`](Self::generate) instead.
    pub fn build(self) -> Result<RosettaConfig, ConfigError> {
        let mut files: HashMap<LanguageIdentifier, PathBuf> = self
            .files
            .into_iter()
            .map(|(lang, path)| {
                let lang = match lang.parse::<LanguageIdentifier>() {
                    Ok(lang) => lang,
                    Err(_) => return Err(ConfigError::InvalidLanguage(lang.to_string())),
                };

                Ok((lang, path))
            })
            .collect::<Result<_, _>>()?;

        if files.is_empty() {
            return Err(ConfigError::MissingSource);
        }

        let fallback = match self.fallback {
            Some(lang) => match lang.parse::<LanguageIdentifier>() {
                Ok(lang) => match files.remove_entry(&lang) {
                    Some(entry) => entry,
                    None => return Err(ConfigError::InvalidFallback),
                },
                Err(_) => return Err(ConfigError::InvalidLanguage(lang.to_string())),
            },
            None => return Err(ConfigError::MissingFallback),
        };

        Ok(RosettaConfig {
            fallback,
            others: files,
            name: self.name.unwrap_or_else(|| "Lang".to_string()),
            output: self.output,
        })
    }
}

/// Configuration for Rosetta code generation
///
/// A [`RosettaBuilder`] is provided to construct and validate configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RosettaConfig {
    pub fallback: (LanguageIdentifier, PathBuf),
    pub others: HashMap<LanguageIdentifier, PathBuf>,
    pub name: String,
    pub output: Option<PathBuf>,
}

impl RosettaConfig {
    /// Get an empty [`RosettaBuilder`]
    pub fn builder() -> RosettaBuilder {
        RosettaBuilder::default()
    }
}

/// Error type returned when the configuration passed to [`RosettaBuilder`] is invalid
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ConfigError {
    /// Invalid language identifier
    #[error("`{0}` is not a valid language identifier")]
    InvalidLanguage(String),
    /// No source provided
    #[error("at least one source is required")]
    MissingSource,
    /// No fallback language provided
    #[error("a fallback language must be provided")]
    MissingFallback,
    /// The fallback language doesn't match any source
    #[error("no source corresponding to the fallback language was found")]
    InvalidFallback,
}
