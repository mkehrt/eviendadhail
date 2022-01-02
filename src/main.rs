use {
    argh, serde, serde_yaml, std::cmp::Ordering, std::collections::HashMap, std::fmt,
    std::fs::File, std::io, std::io::Read as _, std::path::PathBuf, std::str::FromStr,
};

#[derive(argh::FromArgs)]
/// Convert a lexicon YAML file to LaTeX
struct Args {
    /// file containing prelude LaTeX.
    #[argh(option)]
    prelude: PathBuf,
    /// file containing YAML.
    #[argh(option)]
    words: PathBuf,
    /// file containing postlude LaTeX.
    #[argh(option)]
    postlude: PathBuf,
    /// output LaTeX file.
    #[argh(option)]
    output: PathBuf,
}

#[derive(Debug)]
enum LexiconError {
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

#[derive(serde::Deserialize, Debug, Eq, Ord, PartialEq)]
struct Entry {
    word: Option<String>,
    pos: Option<String>,
    defs: Option<Vec<String>>,
    etym: Option<String>,
    notes: Option<String>,
}

impl Entry {
    pub fn is_complete(&self) -> bool {
        self.word.is_some()
            && self.word.unwrap().len() > 0
            && self.pos.is_some()
            && self.defs.is_some()
            && !self.defs.as_ref().unwrap().is_empty()
        // Etym is not a necessary field.
        // Notes is not a necessary field.
    }

    pub fn heading(&self) -> char {
        self.pos.unwrap().chars().collect::<Vec<_>>()[0]
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.word, other.word) {
            (Some(self_word), Some(other_word)) => self_word.partial_cmp(&other_word),
            _ => None,
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        assert!(self.is_complete());

        write!(
            f,
            "\\entry{{{:}}}{{{:}}}{{\n",
            self.word.unwrap(),
            self.pos.unwrap()
        )?;

        let defs = self.defs.unwrap();
        if defs.len() == 1 {
            write!(f, "{{{:}}}", defs[0])?;
        } else {
            for (i, def) in defs.iter().enumerate() {
                write!(f, "\\textbf{{{:}.}} {{{:}}}", i, def)?;
            }
        }

        write!(f, "}}");
        Ok(())
    }
}

#[derive(Eq, Ord, PartialEq)]
struct Section {
    heading: char,
    entries: Vec<Entry>,
}

impl Section {
    fn new(heading: char, entries: Vec<Entry>) -> Self {
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
        for entry in self.entries {
            write!(f, "{:}", entry)?;
        }
        Ok(())
    }
}
struct Sections(Vec<Section>);

impl FromStr for Sections {
    type Err = LexiconError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let entries: Vec<Entry> = serde_yaml::from_str(str)?;
        let entries_by_heading: HashMap<char, Vec<Entry>> = HashMap::new();
        for entry in entries {
            if entry.is_complete() {
                let heading = entry.heading();
                let mut entries = entries_by_heading.entry(heading).or_insert(Vec::new());
                entries.push(entry);
            } else {
                Err(entry)?;
            }
        }

        let mut sections = Vec::new();
        for (heading, entries) in entries_by_heading.into_iter() {
            let section = Section::new(heading, entries);
            sections.push(section);
        }
        sections.sort();

        Ok(Sections(sections))
    }
}

impl fmt::Display for Sections {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for section in self.0 {
            write!(f, "{:}", section)?;
        }
        Ok(())
    }
}

fn main() -> Result<(), LexiconError> {
    let args: Args = argh::from_env();

    let mut prelude_file = File::open(args.prelude)?;
    let mut words_file = File::open(args.words)?;
    let mut postlude_file = File::open(args.postlude)?;
    let mut output_file = File::create(args.output)?;

    let mut words = String::new();
    words_file.read_to_string(&mut words)?;
    let sections = Sections::from_str(words.as_str())?;

    io::copy(&mut prelude_file, &mut output_file)?;
    write!(output_file, "{:}", sections)?;
    io::copy(&mut postlude_file, &mut output_file)?;

    Ok(())
}
