use structopt::StructOpt;
use std::fs;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
  let args = Cli::from_args();
  let result = fs::read_to_string(&args.path);
  let content = match result {
      Ok(content) => { content }
      Err(error) => { panic!("Error: {}", error); }
  };
  for line in content.lines() {
    if line.contains(&args.pattern) {
        println!("{}", line);
    }
  }
}
