use std::{
    collections::HashMap,
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use tinyjson::JsonValue;

use crate::{error::BuildError, gen, parser, LanguageId};

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

        if cfg!(feature = "rustfmt") {
            rustfmt(&output)?;
        }

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
    use crate::{error::ConfigError, LanguageId, RosettaBuilder};

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
