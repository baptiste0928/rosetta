use std::{
    collections::HashMap,
    env,
    fmt::{self, Display},
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
};

use tinyjson::JsonValue;

use crate::{
    error::{BuildError, ConfigError},
    gen, parser,
};

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
    fn build(self) -> Result<RosettaConfig, ConfigError> {
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

        Ok(RosettaConfig {
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
pub(crate) struct LanguageId(pub String);

impl LanguageId {
    pub(crate) fn value(&self) -> &str {
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

/// Configuration for Rosetta code generation
///
/// A [`RosettaBuilder`] is provided to construct and validate configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RosettaConfig {
    pub fallback: (LanguageId, PathBuf),
    pub others: HashMap<LanguageId, PathBuf>,
    pub name: String,
    pub output: Option<PathBuf>,
}

impl RosettaConfig {
    /// Returns a list of the languages
    pub fn languages(&self) -> Vec<&LanguageId> {
        let mut languages: Vec<&LanguageId> =
            self.others.iter().map(|(language, _)| language).collect();
        languages.push(&self.fallback.0);
        languages
    }

    /// Generate locale files and write them to the output location
    pub fn generate(&self) -> Result<(), BuildError> {
        let fallback_content = open_file(&self.fallback.1)?;
        let mut parsed = parser::TranslationData::from_fallback(fallback_content)?;
        println!(
            "cargo:rerun-if-changed={}",
            self.fallback.1.to_string_lossy()
        );

        for (language, path) in &self.others {
            let content = open_file(path)?;
            parsed.parse_file(language.clone(), content)?;
            println!("cargo:rerun-if-changed={}", path.to_string_lossy());
        }

        let generated = gen::CodeGenerator::new(&parsed, self).generate();

        let output = match &self.output {
            Some(path) => path.clone(),
            None => Path::new(&env::var("OUT_DIR")?).join("rosetta_output.rs"),
        };

        let mut file = File::create(&output)?;
        file.write_all(generated.to_string().as_bytes())?;

        #[cfg(feature = "rustfmt")]
        rustfmt(&output)?;

        Ok(())
    }
}

/// Open a file and read its content as a JSON [`JsonValue`]
fn open_file(path: &Path) -> Result<JsonValue, BuildError> {
    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            return Err(BuildError::FileRead {
                file: path.to_path_buf(),
                source: error,
            })
        }
    };

    match content.parse::<JsonValue>() {
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

#[cfg(test)]
mod tests {
    use super::RosettaConfig;
    use crate::{
        builder::{LanguageId, RosettaBuilder},
        error::ConfigError,
    };

    use std::path::PathBuf;

    use maplit::hashmap;

    #[test]
    fn config_simple() -> Result<(), Box<dyn std::error::Error>> {
        let config = RosettaBuilder::default()
            .source("en", "translations/en.json")
            .source("fr", "translations/fr.json")
            .fallback("en")
            .build()?;

        let expected = RosettaConfig {
            fallback: (
                LanguageId("en".into()),
                PathBuf::from("translations/en.json"),
            ),
            others: hashmap! { LanguageId("fr".into()) => PathBuf::from("translations/fr.json") },
            name: "Lang".to_string(),
            output: None,
        };

        assert_eq!(config, expected);

        Ok(())
    }

    #[test]
    fn config_missing_source() {
        let config = RosettaBuilder::default().build();
        assert_eq!(config, Err(ConfigError::MissingSource));
    }

    #[test]
    fn config_invalid_language() {
        let config = RosettaBuilder::default()
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
        let config = RosettaBuilder::default()
            .source("en", "translations/en.json")
            .source("fr", "translations/fr.json")
            .build();

        assert_eq!(config, Err(ConfigError::MissingFallback));
    }

    #[test]
    fn config_invalid_fallback() {
        let config = RosettaBuilder::default()
            .source("en", "translations/en.json")
            .source("fr", "translations/fr.json")
            .fallback("de")
            .build();

        assert_eq!(config, Err(ConfigError::InvalidFallback));
    }
}
