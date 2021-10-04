//! # Rosetta i18n library
//!
//! Rosetta is an easy-to-use and opinionated internationalization (i18n) library for Rust, based on code generation.

use std::borrow::Cow;

pub mod provider;

/// Include the generated translations.
///
/// The generated code will be included in the file as if it were a direct element of it.
/// It is recommended to wrap the generated code in its own module:
///
/// ```ignore
/// mod translations {
///     rosetta_18n::include_translations!();
/// }
/// ```
///
/// This only works if the `rosetta-build` output file has been unmodified.
/// Otherwise, use the following pattern to include the file:
///
/// ```ignore
/// include!("/relative/path/to/rosetta_output.rs");
/// ```
#[macro_export]
macro_rules! include_translations {
    () => {
        include!(concat!(env!("OUT_DIR"), "/rosetta_output.rs"));
    };
}

/// ISO 639-1 language identifier.
///
/// This type holds a string representing a language in the [ISO 693-1] format (two-letter code).
/// The inner value is stored in a [`Cow`] to avoid allocation when possible.
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
pub struct LanguageId<'a>(Cow<'a, str>);

impl<'a> LanguageId<'a> {
    /// Initialize a new [`LanguageId`] from a string reference.
    ///
    /// The provided value should be an [ISO 693-1] encoded language id.
    ///
    /// [ISO 693-1]: https://en.wikipedia.org/wiki/ISO_639-1
    pub fn new(value: &'a str) -> Self {
        Self(Cow::Borrowed(value))
    }

    /// Initialize a new [`LanguageId`] from an owned [String].
    ///
    /// The provided value should be an [ISO 693-1] encoded language id.
    ///
    /// [ISO 693-1]: https://en.wikipedia.org/wiki/ISO_639-1
    pub fn from_string(value: String) -> Self {
        Self(Cow::Owned(value))
    }

    /// Return a reference of the inner value.
    pub fn value(&self) -> &str {
        &self.0
    }
}
