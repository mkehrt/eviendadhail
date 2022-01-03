use {
    crate::types::Entry,
    serde_yaml,
    std::io,
};

#[derive(Debug)]
pub enum LexiconError {
    SerdeYaml(serde_yaml::Error),
    IncompleteEntry(Entry),
    IoError(io::Error),
}

impl From<serde_yaml::Error> for LexiconError {
    fn from(err: serde_yaml::Error) -> Self {
        Self::SerdeYaml(err)
    }
}

impl From<Entry> for LexiconError {
    fn from(entry: Entry) -> Self {
        Self::IncompleteEntry(entry)
    }
}

impl From<io::Error> for LexiconError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}
