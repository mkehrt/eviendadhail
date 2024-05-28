use json5format::{FormatOptions, Json5Format, ParsedDocument};
use lexicon::error::LexiconError;

pub fn format(unformatted: &str) -> Result<String, LexiconError> {
    let filename = "words.json5".to_string();
    let parsed_document = ParsedDocument::from_str(&unformatted, Some(filename))?;

    let format_options = FormatOptions::default();
    let format = Json5Format::with_options(format_options)?;
    let string = format.to_string(&parsed_document)?;

    Ok(string)
}