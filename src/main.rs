use {
    argh, serde,
    std::{fs::File, io, path::PathBuf},
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

#[derive(serde::Deserialize)]
struct Entry {
    word: Option<String>,
    pos: Option<String>,
    defs: Option<Vec<String>>,
    etym: Option<String>,
    notes: Option<String>,
}

fn main() -> io::Result<()> {
    let args: Args = argh::from_env();

    let mut prelude = File::open(args.prelude)?;
    let mut words = File::open(args.words)?;
    let mut postlude = File::open(args.postlude)?;
    let mut output = File::create(args.output)?;

    io::copy(&mut prelude, &mut output)?;

    //generate_lexicon_entries()

    io::copy(&mut postlude, &mut output)?;

    Ok(())
}
