# rosetta-i18n
[![Crates.io](https://img.shields.io/crates/v/rosetta-i18n)](https://crates.io/crates/rosetta-i18n)
[![dependency status](https://deps.rs/repo/github/baptiste0928/rosetta/status.svg)](https://deps.rs/repo/github/baptiste0928/rosetta)
[![docs.rs](https://img.shields.io/docsrs/rosetta-i18n)](https://docs.rs/rosetta-i18n/)
[![CI](https://github.com/baptiste0928/rosetta/actions/workflows/ci.yaml/badge.svg?event=push)](https://github.com/baptiste0928/rosetta/actions/workflows/ci.yaml)

**rosetta-i18n** is an easy-to-use and opinionated Rust internationalization (i18n) library powered by code generation.

```rust
rosetta_i18n::include_translations!();

println!(Lang::En.hello("world"));  // Hello, world!
```

**[Documentation](https://baptiste0928.github.io/rosetta/)**

## Features
- **No runtime errors.** Translation files are parsed at build time, so your code will never fail due to translations anymore.
- **No dependencies.** This crate aims to have the smallest runtime overheat compared to raw strings. There is no additional dependencies at runtime.
- **Standard JSON format.** Translations are written in JSON file with a syntax used by many other i18n libraries. Therefore, most translation services support it out of the box.
- **String formatting** is supported.

## Installation
Rosetta is separated into two crates, `rosetta-i18n` and `rosetta-build`. To install both, add the following to your `Cargo.toml`:

```toml
[dependencies]
rosetta-i18n = "0.1"

[build-dependencies]
rosetta-build = "0.1"
```

## Documentation

The documentation is available on https://baptiste0928.github.io/rosetta/.

You can also read the API documentation on *docs.rs*: [`rosetta-i18n`](https://docs.rs/rosetta-i18n/)
and [`rosetta-build`](https://docs.rs/rosetta-build/).

## Contributing
There is no particular contribution guidelines, feel free to open a new PR to improve the code. If you want to introduce a new feature, please create an issue before.
