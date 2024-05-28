use {crate::serde, crate::types, std::io};

#[derive(Debug)]
pub enum LexiconError {
    SerdeJson5(serde_json5::Error),
    FormatJson5(json5format::Error),
    UnexpectedJson5Element,
    IncompleteEntry(types::Entry),
    IoError(io::Error),
    InvalidAscii(serde::Entry),
}

impl From<serde_json5::Error> for LexiconError {
    fn from(err: serde_json5::Error) -> Self {
        Self::SerdeJson5(err)
    }
}

impl From<json5format::Error> for LexiconError {
    fn from(err: json5format::Error) -> Self {
        Self::FormatJson5(err)
    }
}

impl From<types::Entry> for LexiconError {
    fn from(entry: types::Entry) -> Self {
        Self::IncompleteEntry(entry)
    }
}

impl From<io::Error> for LexiconError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}
