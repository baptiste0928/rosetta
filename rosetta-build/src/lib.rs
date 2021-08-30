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

use std::path::PathBuf;

use thiserror::Error;
use unic_langid::LanguageIdentifier;

/// Helper function that return an empty [`RosettaBuilder`]
pub fn config() -> RosettaBuilder {
    RosettaBuilder::new()
}

/// Builder used to configure Rosetta code generation.
///
/// Please see [Getting started] on the GitHub repository for usage instructions.
#[derive(Debug, PartialEq, Eq)]
pub struct RosettaBuilder {
    files: Vec<(String, PathBuf)>,
    fallback: Option<String>,
    output: Option<PathBuf>,
}

impl RosettaBuilder {
    /// Initialize an empty builder
    pub fn new() -> Self {
        RosettaBuilder {
            files: Vec::new(),
            fallback: None,
            output: None,
        }
    }

    /// Register a new translation source
    pub fn source(mut self, lang: impl Into<String>, path: impl Into<String>) -> Self {
        self.files.push((lang.into(), PathBuf::from(path.into())));
        self
    }

    /// Register the fallback locale
    pub fn fallback(mut self, lang: impl Into<String>) -> Self {
        self.fallback = Some(lang.into());
        self
    }

    /// Change the default output of generated files
    pub fn output(mut self, path: impl Into<String>) -> Self {
        self.output = Some(PathBuf::from(path.into()));
        self
    }

    /// Generate locale files and write them to the output location
    pub fn generate(self) {
        let _ = self.build();
    }

    /// Validate configuration a returns a [`RosettaConfig`]
    ///
    /// This method is not publicly exposed, users should use `generate` instead.
    pub(crate) fn build(self) -> Result<RosettaConfig, ConfigError> {
        let files = self
            .files
            .into_iter()
            .map(|(lang, path)| {
                let lang = match lang.parse::<LanguageIdentifier>() {
                    Ok(lang) => lang,
                    Err(_) => return Err(ConfigError::InvalidLanguage(lang.to_string())),
                };

                Ok((lang, path))
            })
            .collect::<Result<Vec<_>, _>>()?;

        if files.len() == 0 {
            return Err(ConfigError::MissingSource);
        }

        let fallback = match self.fallback {
            Some(lang) => match lang.parse::<LanguageIdentifier>() {
                Ok(lang) => lang,
                Err(_) => return Err(ConfigError::InvalidLanguage(lang.to_string())),
            },
            None => return Err(ConfigError::MissingFallback),
        };

        if files.iter().find(|(lang, _)| lang == &fallback).is_none() {
            return Err(ConfigError::InvalidFallback);
        }

        let output = match self.output {
            Some(output) => output,
            None => {
                let dest = std::env::var("OUT_DIR").unwrap();
                PathBuf::from(dest).join("rosetta.rs")
            }
        };

        Ok(RosettaConfig {
            files,
            fallback,
            output,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct RosettaConfig {
    files: Vec<(LanguageIdentifier, PathBuf)>,
    fallback: LanguageIdentifier,
    output: PathBuf,
}

/// Error type returned when the configuration passed to [`RosettaBuilder`] is invalid
#[derive(Debug, Error)]
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
