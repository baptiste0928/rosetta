//! Code generation for the Rosetta i18n library.
//!
//! # Usage
//! Code generation works within [build script]. You only need to configure source files and
//! the fallback language. Read the [README] in the GitHub repository for more information.
//!
//! ```no_run
//! rosetta_build::config()
//!     .source("fr", "locales/fr.json")
//!     .source("en", "locales/en.json")
//!     .fallback("en")
//!     .generate();
//! ```
//!
//! [build script]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
//! [README]: https://github.com/baptiste0928/rosetta#readme

pub mod error;

mod builder;
mod gen;
mod parser;

pub use crate::builder::{config, RosettaBuilder};
