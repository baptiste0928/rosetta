//! Easy-to-use i18n library for Rust, based on code generation.
//!
//! ## Usage
//! Please read the [documentation] to learn how to use this library.
//!
//! ```ignore
//! mod translations {
//!     rosetta_i18n::include_translations!();
//! }
//!
//! fn main() {
//!     assert_eq!(Lang::En.hello(), "Hello world!");
//! }
//! ```
//!
//! ## Serde support
//! This crate provide serialization and deserialization of languages types with Serde.
//! The `serde` feature must be enabled.
//!
//! [documentation]: https://baptiste0928.github.io/rosetta/
#![cfg_attr(docsrs, feature(doc_cfg))]

use std::borrow::Cow;

#[doc(hidden)]
pub mod provider;
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub mod serde_helpers;

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

/// Trait implemented by languages structs generated by `rosetta-build`.
pub trait Language: Sized {
    /// Initialize this type from a [`LanguageId`].
    ///
    /// The method returns [`None`] if the provided language id is not supported
    /// by the struct.
    fn from_language_id(language_id: &LanguageId) -> Option<Self>;
    /// Convert this struct to a [`LanguageId`].
    fn language_id(&self) -> LanguageId;
    /// Get the fallback language of this type.
    ///
    /// This fallback value can be used like a default value.
    fn fallback() -> Self;
}

/// Generic language type that implement the [`Language`] trait.
///
/// This type can be used as a default generic type when sharing models between multiple
/// crates that does not necessarily use translations.
///
/// ## Panics
/// The [`fallback`] method of the [`Language`] trait is not implemented and will panic if called.
///
/// [`fallback`]: Language::fallback
pub struct GenericLanguage(String);

impl Language for GenericLanguage {
    fn from_language_id(language_id: &LanguageId) -> Option<Self> {
        Some(Self(language_id.value().into()))
    }

    fn language_id(&self) -> LanguageId {
        LanguageId::new(&self.0)
    }

    fn fallback() -> Self {
        unimplemented!("GenericLanguage has no fallback language")
    }
}

/// ISO 639-1 language identifier.
///
/// This type holds a string representing a language in the [ISO 693-1] format (two-letter code).
/// The inner value is stored in a [`Cow`] to avoid allocation when possible.
///
/// ## Validation
/// The type inner value is not validated unless the [`validate`] method is used to initialize the instance.
/// Generally, you should use this method to initialize this type.
///
/// The performed validation only checks that the provided *looks like* an [ISO 693-1]language identifier
/// (2 character alphanumeric ascii string).
///
/// ## Serde support
/// This type implements the `Serialize` and `Deserialize` traits if the `serde` feature is enabled.
/// Deserialization will fail if the value is not an ISO 639-1 language identifier.
///
/// ## Example
/// ```
/// use rosetta_i18n::LanguageId;
///
/// let language_id = LanguageId::new("fr");
/// assert_eq!(language_id.value(), "fr");
///
/// let language_id = LanguageId::validate("fr");
/// assert!(language_id.is_some());
/// ```
///
/// [ISO 693-1]: https://en.wikipedia.org/wiki/ISO_639-1
/// [`validate`]: LanguageId::validate
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LanguageId<'a>(Cow<'a, str>);

impl<'a> LanguageId<'a> {
    /// Initialize a new valid [`LanguageId`].
    ///
    /// Unlike [`new`], this method ensures that the provided
    /// value is a valid [ISO 693-1] encoded language id.
    ///
    /// ```
    /// # use rosetta_i18n::LanguageId;
    /// assert!(LanguageId::validate("fr").is_some());
    /// assert!(LanguageId::validate("invalid").is_none());
    /// ```
    ///
    /// [`new`]: LanguageId::new
    /// [ISO 693-1]: https://en.wikipedia.org/wiki/ISO_639-1
    pub fn validate(value: &str) -> Option<Self> {
        let valid_length = value.len() == 2;
        let ascii_alphabetic = value.chars().all(|c| c.is_ascii_alphabetic());

        if valid_length && ascii_alphabetic {
            Some(Self(Cow::Owned(value.to_ascii_lowercase())))
        } else {
            None
        }
    }

    /// Initialize a new [`LanguageId`] from a string.
    ///
    /// The provided value must be an [ISO 693-1] encoded language id.
    /// If you want to validate the value, use [`validate`] instead.
    ///
    /// ```
    /// # use rosetta_i18n::LanguageId;
    /// let language_id = LanguageId::new("en");
    /// assert_eq!(language_id.value(), "en");
    /// ```
    ///
    /// [ISO 693-1]: https://en.wikipedia.org/wiki/ISO_639-1
    /// [`validate`]: LanguageId::validate
    pub fn new(value: impl Into<Cow<'a, str>>) -> Self {
        Self(value.into())
    }

    /// Return a reference of the inner value.
    pub fn value(&self) -> &str {
        &self.0
    }

    /// Convert the type into a [`String`].
    pub fn into_inner(self) -> String {
        self.0.into_owned()
    }
}
