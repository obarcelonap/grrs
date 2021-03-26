use structopt::StructOpt;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::io;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
#[derive(Debug)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> io::Result<()> {
    let args = Cli::from_args();
    let f = File::open(args.path)?;
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line?;
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}
