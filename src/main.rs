use std::{io::Read, path::PathBuf};

use clap::Parser;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Sponge soaks up stdin ands writes it to a file or to stdout."
)]
struct Cli {
    output: Option<PathBuf>,
}

fn main() {
    let args = Cli::parse();
    let mut buf = String::new();
    std::io::stdin()
        .read_to_string(&mut buf)
        .expect("Failed to read from stdin");
    match args.output {
        Some(file_path) => std::fs::write(file_path, buf).expect("Failed to write to file path"),
        None => print!("{}", buf),
    }
}
