use {
    argh, lexicon::error, lexicon::serde, std::fs::File, std::io::Write as _, std::path::PathBuf,
};

mod format;

#[derive(argh::FromArgs)]
/// In place reformat a json5 file
struct Args {
    /// file containing json5.
    #[argh(option)]
    words: PathBuf,
}

fn main() -> Result<(), error::LexiconError> {
    let args: Args = argh::from_env();

    let mut words_file = File::open(&args.words)?;
    let mut entries = serde::serde_entries_from_reader(&mut words_file)?;
    drop(words_file);

    entries.sort();
    let json5_entries = serde_json5::to_string(&entries)?;

    let formatted_json5_entries = format::format(&json5_entries)?;

    let mut words_file = File::create(&args.words)?;
    writeln!(words_file, "{:}", formatted_json5_entries)?;

    Ok(())
}
