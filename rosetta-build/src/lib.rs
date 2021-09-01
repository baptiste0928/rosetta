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

pub mod gen;
pub mod parser;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use icu_locid::LanguageIdentifier;
use serde_json::Value;
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
    pub fn generate(self) -> Result<(), BuildError> {
        self.build()?.generate()?;
        Ok(())
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

    /// Returns a list of the languages
    pub fn languages(&self) -> Vec<&LanguageIdentifier> {
        let mut languages: Vec<&LanguageIdentifier> =
            self.others.iter().map(|(language, _)| language).collect();
        languages.push(&self.fallback.0);
        languages
    }

    /// Generate locale files and write them to the output location
    pub fn generate(&self) -> Result<(), BuildError> {
        let fallback_content = open_file(&self.fallback.1)?;
        let mut parsed = parser::TranslationData::from_fallback(fallback_content)?;
        println!("cargo:rerun-if-changed={}", self.fallback.1.to_string_lossy());

        for (language, path) in &self.others {
            let content = open_file(path)?;
            parsed.parse_file(language.clone(), content)?;
            println!("cargo:rerun-if-changed={}", path.to_string_lossy());
        }

        let generated = gen::CodeGenerator::new(&parsed, self).generate();

        let output = match &self.output {
            Some(path) => path.clone(),
            None => Path::new(&env::var("OUT_DIR")?).join("rosetta.rs"),
        };

        let mut file = File::create(&output)?;
        file.write_all(generated.to_string().as_bytes())?;

        if cfg!(feature = "rustfmt") {
            rustfmt(&output)?;
        }

        Ok(())
    }
}

/// Open a file and read its content as a JSON [`Value`]
fn open_file(path: &Path) -> Result<Value, BuildError> {
    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            return Err(BuildError::FileRead {
                file: path.to_path_buf(),
                source: error,
            })
        }
    };

    match serde_json::from_str(&content) {
        Ok(parsed) => Ok(parsed),
        Err(error) => Err(BuildError::JsonParse {
            file: path.to_path_buf(),
            source: error,
        }),
    }
}

/// Format a file with rustfmt
#[cfg(feature = "rustfmt")]
fn rustfmt(path: &Path) -> Result<(), BuildError> {
    use std::process::Command;

    Command::new(env::var("RUSTFMT").unwrap_or_else(|_| "rustfmt".to_string()))
        .args(&["--emit", "files"])
        .arg(path)
        .output()
        .map_err(BuildError::Fmt)?;

    Ok(())
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

/// Error type returned when the code generation failed for some reason
#[derive(Debug, Error)]
pub enum BuildError {
    #[error("invalid configuration: {0}")]
    Config(#[from] ConfigError),
    #[error("failed to read `{file}`: {source}")]
    FileRead {
        file: PathBuf,
        source: std::io::Error,
    },
    #[error("failed to write output: {0}")]
    FileWrite(#[from] std::io::Error),
    #[error("failed to load {file}: {source}")]
    JsonParse {
        file: PathBuf,
        source: serde_json::Error,
    },
    #[error("failed to parse translations: {0}")]
    Parse(#[from] parser::ParseError),
    #[error("failed to read env variable: {0}")]
    Var(#[from] env::VarError),
    #[error("failed to run rustfmt: {0}")]
    Fmt(#[source] std::io::Error),
}
