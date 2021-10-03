//! Language data providers.
//!
//! This module contains types and traits for language data providers.
//! Data providers are responsible of providing data to localize strings
//! in given languages, such a plural rules.
//!
//! As Rosetta aims to be simple to use and maintain, we do not provide
//! ready-to-use providers for every languages. We only provide a [`DefaultProvider`]
//! which works for few common latin languages. However, you a free to implement
//! providers for languages you need to support.
//!
//! ## Implementing a provider
//! If you need to support extra languages that are not in the default provider,
//! you should create a type that implement [`LanguageProvider`]. This type
//! can then be used as a generic parameter of language type created by `rosetta-build`.
//!
//! Example implementation:
//! ```
//! use rosetta_i18n::{
//!     LanguageId,
//!     provider::{LanguageProvider, PluralCategory}
//! };
//!
//! /// A provider that only works for French.
//! struct FrenchProvider;
//!
//! impl LanguageProvider for FrenchProvider {
//!     fn from_id(_language_id: &LanguageId) -> Self {
//!         Self
//!     }
//!
//!     fn plural(&self, number: u64) -> PluralCategory {
//!         match number {
//!             0 | 1 => PluralCategory::One,
//!             _ => PluralCategory::Other
//!         }
//!     }
//! }
//! ```
//!
//! ## CLDR Data
//! The reference source for locale data is [Unicode CLDR], an exhaustive dataset
//! containing information about plural cases, number formatting and many more for
//! most languages in the world.
//!
//! Many i18n Rust libraries such as `intl_pluralrules` bundle data from CLDR, and
//! others like `icu4x` must be configured with an external CLDR data source.
//! However, as the CLDR dataset is large and we only need a small subset of it,
//! we did not choose to use it directly: most applications use only a few languages,
//! so implementing a data provider manually is not much work. If you need to use the
//! entire CLDR dataset, Rosetta might not be the good choice for you.
//!
//! If you need to implement a custom language provider, **it is strongly recommended to rely on
//! CLDR data**. You can easily find this online (e.g. [plural rules]).
//!
//! [Unicode CLDR]: https://cldr.unicode.org/
//! [plural rules]: https://unicode-org.github.io/cldr-staging/charts/37/supplemental/language_plural_rules.html

use crate::LanguageId;

/// Trait for language data providers.
///
/// This trait is implemented on types that provide data used
/// to localize strings in a given language. See [`DefaultProvider`]
/// for an example implementation.
pub trait LanguageProvider: Sized {
    /// Initialize the provider from a language identifier.
    ///
    /// This method deliberately cannot fail. If a invalid
    /// value is provided, you should either return a default
    /// or a generic value.
    fn from_id(language_id: &LanguageId) -> Self;

    /// Select the appropriate [`PluralCategory`] for a given number.
    fn plural(&self, number: u64) -> PluralCategory;
}

/// CLDR Plural category.
///
/// This type represent a plural category as defined in [Unicode CLDR Plural Rules].
///
/// [Unicode CLDR Plural Rules]: https://cldr.unicode.org/index/cldr-spec/plural-rules
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PluralCategory {
    /// Zero plural category.
    ///
    /// Used in Arabic, Latvian, and others.
    Zero,
    /// One plural category.
    ///
    /// Used for the singular form in many languages.
    One,
    /// Two plural category.
    ///
    /// Used in Arabic, Hebrew, Slovenian, and others.
    Two,
    /// Few plural category.
    ///
    /// Used in Romanian, Polish, Russian, and others.
    Few,
    /// Many plural category.
    ///
    /// Used in Polish, Russian, Ukrainian, and others.
    Many,
    /// Other plural category, used as a catch-all.
    ///
    /// In some languages, such as Japanese, Chinese, Korean, and Thai, this is the only plural category.
    Other,
}

/// Default built-in data provider.
///
/// This type is a default provider implementation provided for
/// simple use cases or testing purposes. **It only implements
/// few common latin languages.** You should implement [`LanguageProvider`]
/// yourself if you want to support more languages.
///
/// The [`En`](DefaultProvider::En) variant is used when an unknown
/// [`LanguageId`] is provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DefaultProvider {
    /// English
    En,
    /// Spanish
    Es,
    /// French
    Fr,
    /// German
    De,
    /// Italian
    It,
}

impl LanguageProvider for DefaultProvider {
    fn from_id(language_id: &LanguageId) -> Self {
        match language_id.value() {
            "es" => Self::Es,
            "fr" => Self::Fr,
            "de" => Self::De,
            "it" => Self::It,
            _ => Self::En,
        }
    }

    fn plural(&self, number: u64) -> PluralCategory {
        match self {
            Self::En | Self::Es | Self::De | Self::It => match number {
                1 => PluralCategory::One,
                _ => PluralCategory::Other,
            },
            Self::Fr => match number {
                0 | 1 => PluralCategory::One,
                _ => PluralCategory::Other,
            },
        }
    }
}
