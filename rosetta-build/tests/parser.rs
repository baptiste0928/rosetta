use icu_locid_macros::langid;
use maplit::hashmap;
use rosetta_build::parser::*;
use serde_json::json;

#[test]
fn parse_simple() -> Result<(), Box<dyn std::error::Error>> {
    let en = json!({ "hello": "Hello world!" });
    let fr = json!({"hello": "Bonjour le monde !"});

    let mut parsed = TranslationData::from_fallback(en)?;
    parsed.parse_file(langid!("fr"), fr)?;

    assert_eq!(parsed.keys.len(), 1);
    assert!(parsed.keys.get("hello").is_some());

    let expected = TranslationKey::Simple {
        fallback: "Hello world!".to_string(),
        others: hashmap! {
            langid!("fr") => "Bonjour le monde !".to_string()
        },
    };

    assert_eq!(parsed.keys.get("hello").unwrap(), &expected);

    Ok(())
}

#[test]
fn parse_invalid_root() {
    let file = json!("invalid");
    let parsed = TranslationData::from_fallback(file);
    assert_eq!(parsed, Err(ParseError::InvalidRoot));
}

#[test]
fn parse_invalid_value() {
    let file = json!({"hello": ["Hello world"]});
    let parsed = TranslationData::from_fallback(file);
    assert_eq!(
        parsed,
        Err(ParseError::InvalidValue {
            key: "hello".to_string()
        })
    );
}
