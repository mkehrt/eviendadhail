use {
    argh,
    std::fs::File, std::io::Write as _, std::path::PathBuf,
    lexicon::error,
    lexicon::serde,
};

#[derive(argh::FromArgs)]
/// Convert a lexicon YAML file to LaTeX
struct Args {
    /// file containing YAML.
    #[argh(option)]
    words: PathBuf,
}

fn main() -> Result<(), error::LexiconError> {
    let args: Args = argh::from_env();
    
    let mut words_file = File::open(&args.words)?;
    let mut entries = serde::serde_entries_from_reader(&mut words_file)?;
    drop(words_file);

    entries.sort();

    let mut words_file = File::create(&args.words)?;
    for entry in entries {
        let yaml_entry =  serde_yaml::to_string(&entry)?;
        writeln!(words_file, "{:}", yaml_entry)?;
        writeln!(words_file, "",)?;
    }

    Ok(())
}
