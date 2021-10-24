# Overview

Rosetta is an easy-to-use Rust [internationalization](https://en.wikipedia.org/wiki/Internationalization_and_localization)
library powered by code generation. Unlike other libraries, translation files are parsed and embedded into the
resulting binary at build-time. This provide a better developer experience and reduce runtime overheat.


Using your translation files in your project is (almost) as easy as that:
```rust
rosetta_i18n::include_translations!();

println!("{}", Lang::En.hello("world"));  // Hello, world!
```

The following documentation aims to provide an exhaustive guide about using Rosetta in your project.
See [Getting started](./getting_started.md) to get an usage overview.

## Related links
Here are all the links related to the project:

- **[GitHub repository](https://github.com/baptiste0928/rosetta)** - where the development happen, feel free to contribute!
- [`rosetta-i18n` on crates.io](https://crates.io/crates/rosetta-i18n) - main crate containing all useful runtime features.
- [`rosetta-build` on crates.io](https://crates.io/crates/rosetta-build) - crate used for code generation.
- [`rosetta-i18n`](https://docs.rs/rosetta-i18n/) and [`rosetta-build`](https://docs.rs/rosetta-build/) on **docs.rs** - up-to-date API documentation.

> Please give a ⭐ to the GitHub repository if you use Rosetta.

## Support
If you encounter bugs or need help using Rosetta, here's what to do:

- **If you need help with Rosetta**, [open a new discussion](https://github.com/baptiste0928/rosetta/discussions) on the GitHub repository.
- **To report a bug or suggest a new feature**, [open a new issue](https://github.com/baptiste0928/rosetta/issues) on the GitHub repository.

Please do not open issues for help request, this is not the right place for it. Use discussions instead.

## Contributing
Rosetta is free and open-source. You can find the source code on GitHub and open a new issue to report bug or request features.
If you want to improve the code or the documentation, consider opening a [pull request](https://github.com/baptiste0928/rosetta/pulls).

Any contribution is welcome, even the smallest! 🙌
