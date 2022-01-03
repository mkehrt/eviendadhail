use {crate::serde, crate::types, serde_yaml, std::io};

#[derive(Debug)]
pub enum LexiconError {
    SerdeYaml(serde_yaml::Error),
    IncompleteEntry(types::Entry),
    IoError(io::Error),
    InvalidAscii(serde::Entry),
}

impl From<serde_yaml::Error> for LexiconError {
    fn from(err: serde_yaml::Error) -> Self {
        Self::SerdeYaml(err)
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
