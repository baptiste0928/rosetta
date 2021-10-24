# JSON file format

The following is an exhaustive reference of the [JSON](https://en.wikipedia.org/wiki/JSON) file format used for translations.

> **Note:** nested keys are not yet available.

## Simple key
A simple translation key is a static string key without any variable interpolation. The `{` and `}` characters are not allowed.

```json
{
    "simple": "Hello world!"
}
```

## String formatting
You can add variables inside keys to insert dynamic content at runtime. Variable name should be in `snake_case` surrounded by `{` and `}` characters.

```json
{
    "formatted": "I like three things: {first}, {second} and Rust."
}
```

You can add as many parameters as you want. The same parameter can be inserted several times.
Languages that are not fallback languages **must** have the same parameters as the fallback language.
