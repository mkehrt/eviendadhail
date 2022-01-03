use {
    argh, lexicon::error, lexicon::serde, lexicon::types, std::fs::File, std::io,
    std::io::Write as _, std::path::PathBuf,
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

fn report_invalid_entries(entries: &Vec<serde::Entry>) {
    eprintln!("The following lexicon entries were not complete and could not be output:");
    for entry in entries {
        eprintln!("{:?}", entry);
    }
}

fn main() -> Result<(), error::LexiconError> {
    let args: Args = argh::from_env();

    let mut words_file = File::open(args.words)?;

    let entries = serde::entries_from_reader(&mut words_file)?;
    let sections = types::Sections::new_from_entries(entries.valid);

    report_invalid_entries(&entries.invalid);

    let mut prelude_file = File::open(args.prelude)?;
    let mut postlude_file = File::open(args.postlude)?;
    let mut output_file = File::create(args.output)?;

    io::copy(&mut prelude_file, &mut output_file)?;
    write!(output_file, "{:}", sections)?;
    io::copy(&mut postlude_file, &mut output_file)?;

    Ok(())
}
