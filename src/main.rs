use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;

use structopt::StructOpt;

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
    let args: Cli = Cli::from_args();
    let file = File::open(&args.path)?;
    let buff = BufReader::new(file);

    buff.lines()
        .for_each(|line| match line {
            Ok(content) if (content.contains(&args.pattern)) => println!("{}", content),
            Err(error) => { panic!("Can't deal with {}, just exit here", error); }
            _ => {}
        });

    Ok(())
}
