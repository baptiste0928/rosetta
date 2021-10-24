# Getting started

The following guide explains how to use `rosetta-i18` and `rosetta-build` to manage your translations.
Please refer to other sections in this documentation or to the API documentation for in depth explanations.

## Installation
Rosetta is separated into two crates: `rosetta-i18n` and `rosetta-build`. To install both, add the following to your `Cargo.toml`:

```toml
[dependencies]
rosetta-i18n = "0.1"

[build-dependencies]
rosetta-build = "0.1"
```

`rosetta-build` is used inside a build script and must be a build dependency.

## Writing translations files
Rosetta use JSON translation files, which is similar to the one used by many other translation libraries and this widely supported.
We need to put these files somewhere, for example in a `/locales` directory.

`locales/en.json`
```json
{
    "hello": "Hello world!",
    "hello_name": "Hello {name}!"
}
```

In this example, we defined two keys, `hello` and `hello_name`. The first is a static string, whereas the second contains the `name` variable which will be
replaced at runtime by the value of your choice.

Create a file for each language you want to be supported by your application. It is not required that all files contain all keys: we will define the fallback language later.

## Generating code from translation files
It is now that the magic happens. Rosetta lets you generate a Rust type from your translation files.
For that, it use a [build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html) which will be run each time you edit a translation file.

We need to create a `build.rs` file at the root folder of the crate (same folder as the `Cargo.toml` file).

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

This script use the `rosetta_build` crate. In this example, we define two languages with [ISO 639-1](https://en.wikipedia.org/wiki/ISO_639-1)
language identifiers: `en` and `fr`. The `en` language is defined as fallback and will be used if a key is not defined in other languages.

The `.generate()` method is responsible for code generation. By default, the output file will be generated in a folder inside the `target` directory (`OUT_DIR` env variable). 

## Using the generated type
The generated type (named `Lang` except if you defined another name - see the previous section) must be included in your code with the `include_translations`
macro. A good practice is to isolate it in a dedicated module.

Each translation key is transformed into a method, and each language into an enum variant. Parameters are sorted alphabetically to avoid silent breaking changes
when reordering.

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

