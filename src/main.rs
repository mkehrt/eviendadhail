use {Serde, Argh, std::{io::File, pathbuf::PathBuf)}

#[derive()Argh::fromArgs)]
struct Args {
  prelude: PathBuf,
  words: PathBuf,
  postlude: PathBuf,
  output: PathBuf,
}

#[derive(Serde::Deserialize)]
struct Entry {
  word: String,
  pos: String,
  defs: Vec<String>
  etym: String
  notes: Option<String>
}

fn main() -> io::Result<()>) {
    let args = Args::from_env());
    
    let mut prelude = File::open(args.prelude)?;
    let mut words = File::open(args.words)?;
    let mut postlude = File::open(args.postlude)?;
    let mut output = File::create(args.output)?;

    output.write_all(prelude)?;

    output.write_all(postlude)?;
}
