fn main() -> Result<(), Box<dyn std::error::Error>> {
    rosetta_build::config()
        .source("en", "locales/en.json")
        .source("fr", "locales/fr.json")
        .fallback("en")
        .generate()?;

    Ok(())
}