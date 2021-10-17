# rosetta-i18n
[![Crates.io](https://img.shields.io/crates/v/rosetta-i18n)](https://crates.io/crates/rosetta-i18n)
[![docs.rs](https://img.shields.io/docsrs/rosetta-i18n)](https://docs.rs/rosetta-i18n/)
[![CI](https://github.com/baptiste0928/rosetta/actions/workflows/ci.yaml/badge.svg?event=push)](https://github.com/baptiste0928/rosetta/actions/workflows/ci.yaml)

**rosetta-i18n** is an easy-to-use and opinionated Rust internationalization (i18n) library powered by code generation.

```rust
rosetta_i18n::include_translations!();

println!(Lang::En.hello("world"));  // Hello, world!
```

## Features
- **No runtime errors.** Translation files are parsed at build time, so your code will never fail due to translations anymore.
- **No dependencies.** This crate aims to have the smallest runtime overheat compared to raw strings. There is no additional dependencies at runtime.
- **Standard JSON format.** Translations are written in JSON file with a syntax used by many other i18n libraries. Therefore, most translation services support it out of the box.
- **String formatting** is supported and 100% safe.

## Installation
Rosetta is separated into two crates, `rosetta-i18n` and `rosetta-build`. To install both, add the following to your `Cargo.toml`:

```toml
[dependencies]
rosetta-i18n = "0.1"

[build-dependencies]
rosetta-build = "0.1"
```

## Getting started
The following guide explains how to use `rosetta-i18` to manage your translations.
Please refer to the crate documentation for in depth explanations about the exposed types.

### 1- Writing translation files
First, we have to write translation files in JSON format, for example in a `/locales` directory.
The format is similar to other translations libraries.

`locales/en.json`
```json
{
    "hello": "Hello world!",
    "hello_name": "Hello {name}!"
}
```

In this example, we defined two keys, `hello` and `hello_name`. The first is a static string, whereas the second contains the `name` variable which will be
replaced at runtime by the value of your choice. Note that nested key are not (yet?) supported.

Then, create a file for each language you want. It is not required that all files contain all keys. We will define a fallback language later.

### 2- Generate code from translation files
Code generation use a [build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html), which will be run each time you edit a translation file.
Let's create a `build.rs` file at the root folder of your crate (same folder as the `Cargo.toml` file).

`build.rs`
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    rosetta_build::config()
        .source("en", "locales/en.json")
        .source("fr", "locales/fr.json")
        .fallback("en")
        .generate()?;

    Ok(())
}
```

This script will use the `rosetta_build` crate. In this example, we define two languages: `en` and `fr`. Languages identifiers must be in the
[ISO 639-1](https://en.wikipedia.org/wiki/ISO_639-1) format (two-letters code). The `en` language is defined as fallback, so if a key is not
defined in our `fr.json` file, the English string will be used.
The output type name can also be customized with the `.name()` method, as well as the output folder with the `.output()` method.

Then, call the `.generate()` method to generate the code. By default, it will be generated in a folder inside the `target` directory (`OUT_DIR` env variable).

### 3- Use generated type
The generated type (named `Lang` except if you defined another name - see the previous section) must be included in your code with the `include_translations`
macro. A good practice is to isolate it in a dedicated module.

Each translation key is transformed into a method, and each language into an enum variant.

`src/main.rs`
```rust
mod translations {
    rosetta_i18n::include_translations!();
}

fn main() {
    use translations::Lang;

    println!("{}", Lang::En.hello());  // Hello world!
    println!("{}", Lang::En.hello_name("Rust"));  // Hello Rust!
}
```

## Contributing
There is no particular contribution guidelines, feel free to open a new PR to improve the code. If you want to introduce a new feature, please create an issue before.
