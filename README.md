# rosetta-i18n

`rosetta-i18n` is a Rust internationalization (i18n) library powered by code generation.

**Disclaimer:** This crate is currently work in progress. Breaking changes may happen at any time before the crate is published on crates.io.
If you are using it, it is recommended to link to a specific commit in your Cargo.toml file to avoid unwanted breaking change. Any feedback is welcome.


## Features
- **No runtime errors.** Translation files are parsed at build time, so your code will never fail due to translations anymore.
- **No dependencies.** This crate aims to have the smallest runtime overheat compared to raw strings. There is no additional dependencies at runtime.
- **Standard JSON format.** Translations are written in JSON file with a syntax used by many other i18n libraries. Therefore, most translation services support it out of the box.
- **Plurals and variable interpolation** are fully supported. (TODO)

## Installation
While this crate is not published on *crates.io*, you should use it as a git dependency. It is recommended to link to a specific commit to avoid unwanted breaking changes.

```toml
# Cargo.toml
[dependencies]
rosetta-i18n = { git = "https://github.com/baptiste0928/rosetta", rev = "commit" }

[build-dependencies]
rosetta-build = { git = "https://github.com/baptiste0928/rosetta", rev = "commit" }
```

## Example usage
`locales/en.json`
```json
{
    "hello": "Hello world!"
}
```

`build.rs`
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    rosetta_build::config()
        .source("fr", "locales/fr.json")
        .source("en", "locales/en.json")
        .fallback("en")
        .generate()?;

    Ok(())
}
```

`src/main.rs`
```rust
rosetta_i18n::include_translations!();

fn main() {
    println!("{}", Lang::En.hello());
}
```
