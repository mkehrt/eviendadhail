use {
    serde, serde_yaml, 
    std::cmp::PartialOrd,
    std::cmp::Ord,
    std::cmp::Ordering,
    std::io,
    crate::error,
    crate::types,
};

#[allow(unused)]
#[derive(Debug, Eq, Ord, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Entry {
    word: Option<String>,
    pos: Option<String>,
    def: Option<String>,
    defs: Option<Vec<String>>,
    etym: Option<String>,
    notes: Option<String>,
}

impl Entry {
    fn defs_are_valid(&self) -> bool {
        let def = &self.def;
        let defs = &self.defs;

        let one_is_some = (def.is_some() && !defs.is_some()) || (!def.is_some() && defs.is_some());
        let def_is_valid = def.is_some();
        let defs_is_valid = defs.is_some() && defs.as_ref().unwrap().len() > 0;
        
        one_is_some && (def_is_valid || defs_is_valid) 
    }

    pub fn is_valid(&self) -> bool {
        self.word.is_some()
            && self.word.as_ref().unwrap().len() > 0
            && self.pos.is_some()
            && self.defs_are_valid()
        // Etym is not a necessary field.
        // Notes is not a necessary field.
    }

    fn extract_defs(self) -> Vec<String> {
        if self.def.is_some() {
            vec![self.def.unwrap()]
        } else {
            self.defs.unwrap()
        }
    }

    pub fn to_complete_entry(mut self) -> types::Entry {
        let word = self.word.take().unwrap();
        let pos = self.pos.take().unwrap();
        let defs = self.extract_defs();

        types::Entry::new(word, pos, defs)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (&self.word, &other.word) {
            (Some(self_word), Some(other_word)) => self_word.partial_cmp(other_word),
            (_, _) => None
        }
    }
}

pub fn serde_entries_from_reader<R: io::Read>(reader: &mut R) -> Result<Vec<Entry>, error::LexiconError> {
    let serde_entries = serde_yaml::from_reader(reader)?;
    Ok(serde_entries)
}

pub struct Entries {
    pub valid: Vec<types::Entry>,
    pub invalid: Vec<Entry>,
}

pub fn entries_from_reader<R: io::Read>(reader: &mut R) -> Result<Entries, error::LexiconError> {
    let serde_entries: Vec<Entry> = serde_entries_from_reader(reader)?;
    let (valid, invalid): (Vec<Entry>, Vec<Entry>) = serde_entries.into_iter().partition(Entry::is_valid);

    let valid: Vec<_> = valid.into_iter().map(Entry::to_complete_entry).collect();

    Ok(Entries { valid, invalid })
}
