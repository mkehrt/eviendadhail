use {Serde, Argh, std::PathBuf}

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

fn main() {
    println!("Hello, world!");
}
