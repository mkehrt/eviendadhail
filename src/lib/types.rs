use {crate::latex_util, std::cmp::Ordering, std::collections::HashMap, std::fmt};

#[derive(Debug, Eq, Ord, PartialEq)]
pub struct Entry {
    word: String,
    pos: String,
    defs: Vec<String>,
}

impl Entry {
    pub fn new(word: String, pos: String, defs: Vec<String>) -> Self {
        Self { word, pos, defs }
    }

    pub fn heading(&self) -> String {
        latex_util::string_to_base_string(&self.word)
            .chars()
            .next()
            .unwrap()
            .to_uppercase()
            .to_string()
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        latex_util::compare_by_base_string(&self.word, &other.word)
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\\entry{{{:}}}{{{:}}}{{", self.word, self.pos)?;

        if self.defs.len() == 1 {
            write!(f, "{:}", self.defs[0])?;
        } else {
            for (i, def) in self.defs.iter().enumerate() {
                write!(f, "\\textbf{{{:}.}} {:} ", i + 1, def)?;
            }
        }

        write!(f, "}}\n")?;
        Ok(())
    }
}

#[derive(Eq, Ord, PartialEq)]
pub struct Section {
    heading: String,
    entries: Vec<Entry>,
}

impl Section {
    fn new(heading: String, mut entries: Vec<Entry>) -> Self {
        entries.sort();
        Self { heading, entries }
    }
}

impl PartialOrd for Section {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.heading.partial_cmp(&other.heading)
    }
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\\section*{{{:}}}\n", self.heading)?;
        for entry in self.entries.iter() {
            write!(f, "{:}", entry)?;
        }
        Ok(())
    }
}

pub struct Sections(Vec<Section>);

impl Sections {
    pub fn new_from_entries(entries: Vec<Entry>) -> Self {
        let mut entries_by_heading: HashMap<String, Vec<Entry>> = HashMap::new();
        for entry in entries {
            let heading = entry.heading();
            let entries_for_heading = entries_by_heading.entry(heading).or_insert(Vec::new());
            entries_for_heading.push(entry);
        }

        let mut sections = Vec::new();
        for (heading, entries) in entries_by_heading.into_iter() {
            let section = Section::new(heading, entries);
            sections.push(section);
        }
        sections.sort();

        Sections(sections)
    }
}

impl fmt::Display for Sections {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for section in &self.0 {
            write!(f, "{:}", section)?;
        }
        Ok(())
    }
}
