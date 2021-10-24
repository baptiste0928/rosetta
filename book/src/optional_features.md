# Optional features

The `rosetta-i18n` and `rosetta-build` crates allow to turn on some additional features with [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html).
Most of these requires additional dependencies and are not enabled by default.

> To enable a feature, you need to add a `feature` key in the `Cargo.toml` file like the following example:
>
> ```toml
> rosetta-i18n = { version = "0.1", features = ["serde"] }
> ``` 

## `rosetta-i18n`

- `serde`: enable [Serde](https://serde.rs/) support, providing `Serialize` and `Deserialize` implementation for some types. Utility functions to serialize and deserialize
generated types are also provided.

## `rosetta-build`

- `rustfmt` *(enabled by default)*: format generated code with [rustfmt](https://github.com/rust-lang/rustfmt). Disable this feature if `rustfmt` is not installed in your computer.
