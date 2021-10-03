//! # Rosetta i18n library
//!
//! Rosetta is an easy-to-use and opinionated internationalization (i18n) library for Rust, based on code generation.

pub mod provider;

/// ISO 639-1 language identifier.
///
/// This type holds a string representing a language in the [ISO 693-1] format (two-letter code).
///
/// ```
/// use rosetta_i18n::LanguageId;
///
/// let lang_id = LanguageId::new("fr");
/// assert_eq!(lang_id.value(), "fr");
/// ```
///
/// [ISO 693-1]: https://en.wikipedia.org/wiki/ISO_639-1
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LanguageId(String);

impl LanguageId {
    /// Initialize a new [`LanguageId`] from a string.
    ///
    /// The provided value should be an [ISO 693-1] encoded language id.
    ///
    /// [ISO 693-1]: https://en.wikipedia.org/wiki/ISO_639-1
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Return a reference of the inner value.
    pub fn value(&self) -> &str {
        &self.0
    }
}
