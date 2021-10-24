# Build options

The following is an exhaustive reference of all configurable build options.

These options are provided as methods of the [`RosettaBuilder`](https://docs.rs/rosetta-build/*/rosetta_build/struct.RosettaBuilder.html) type.

**Required options :**
- [`.fallback()`](https://docs.rs/rosetta-build/*/rosetta_build/struct.RosettaBuilder.html#method.fallback): register the fallback language with a given language identifier and path
- [`.source()`](https://docs.rs/rosetta-build/*/rosetta_build/struct.RosettaBuilder.html#method.source): register an additional translation source with a given language identifier and path

**Additional options :**
- [`.name()`](https://docs.rs/rosetta-build/*/rosetta_build/struct.RosettaBuilder.html#method.name): use a custom name for the generate type (`Lang` by default)
- [`.output()`](https://docs.rs/rosetta-build/*/rosetta_build/struct.RosettaBuilder.html#method.output): export the type in another output location (`OUT_DIR` by default)

More information in the [`RosettaBuilder` API documentation](https://docs.rs/rosetta-build/*/rosetta_build/struct.RosettaBuilder.html).
