use {
    crate::error, crate::latex_util, crate::types, serde, serde_json5, std::cmp::Ord,
    std::cmp::Ordering, std::cmp::PartialOrd, std::io,
};

#[allow(unused)]
#[derive(Clone, Debug, Eq, Ord, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Entry {
    #[serde(skip_serializing_if = "Option::is_none")]
    word: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pos: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    def: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    defs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    etym: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

    fn all_visible_fields<F: Fn(&String) -> bool>(&self, f: &F) -> bool {
        self.word.as_ref().map_or(true, f)
            && self.pos.as_ref().map_or(true, f)
            && self.def.as_ref().map_or(true, f)
            && self.defs.as_ref().map_or(true, |v| v.iter().all(|s| f(s)))
            && self.etym.as_ref().map_or(true, f)
    }

    pub fn is_ascii(&self) -> bool {
        self.all_visible_fields(&|string| string.chars().all(|c| char::is_ascii(&c)))
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (&self.word, &other.word) {
            (Some(self_word), Some(other_word)) => {
                latex_util::compare_by_base_string(&self_word, other_word)
            }
            (_, _) => None,
        }
    }
}

pub fn serde_entries_from_reader<R: io::Read>(
    reader: &mut R,
) -> Result<Vec<Entry>, error::LexiconError> {
    let serde_entries: Vec<Entry> = serde_json5::from_reader(reader)?;
    for serde_entry in serde_entries.iter() {
        if !serde_entry.is_ascii() {
            return Err(error::LexiconError::InvalidAscii(serde_entry.clone()));
        }
    }
    Ok(serde_entries)
}

pub struct Entries {
    pub valid: Vec<types::Entry>,
    pub invalid: Vec<Entry>,
}

pub fn entries_from_reader<R: io::Read>(reader: &mut R) -> Result<Entries, error::LexiconError> {
    let serde_entries: Vec<Entry> = serde_entries_from_reader(reader)?;
    let (valid, invalid): (Vec<Entry>, Vec<Entry>) =
        serde_entries.into_iter().partition(Entry::is_valid);

    let valid: Vec<_> = valid.into_iter().map(Entry::to_complete_entry).collect();

    Ok(Entries { valid, invalid })
}
