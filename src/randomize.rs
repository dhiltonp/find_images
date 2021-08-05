use std::env::current_dir;
use std::fs::File;
use std::io::{BufRead, BufReader, LineWriter, Write};
use std::path::{Path, PathBuf};
use std::process::exit;

use rand::seq::SliceRandom;
use rand::thread_rng;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    output: PathBuf,
}

fn slurp(input: &Path) -> Vec<String> {
    if let Ok(file) = File::open(&input) {
        let mut retval = Vec::new();

        let buf = BufReader::new(file);
        for line in buf.lines().flatten() {
            println!("{:?}", &line);
            retval.push(line);
        }
        retval
    } else {
        if let Ok(current_dir) = current_dir() {
            eprintln!("unable to read {:?} in {:?}", &input, current_dir);
        }
        exit(1);
    }
}

fn write(output: &Path, lines: Vec<String>) {
    if let Ok(file) = File::create(&output) {
        if let Ok(current_dir) = current_dir() {
            eprintln!("writing output to {:?} in {:?}", &output, current_dir);
        }
        let mut file = LineWriter::new(file);
        for line in lines {
            file.write_all(line.as_bytes())
                .expect("unable to write file!");
            file.write_all(b"\n").expect("unable to write file!");
        }
    } else {
        if let Ok(current_dir) = current_dir() {
            eprintln!("unable to create {:?} in {:?}", &output, current_dir);
        }
        exit(2);
    }
}

fn main() {
    let args = Cli::from_args();

    let mut lines = slurp(&args.output);
    lines.shuffle(&mut thread_rng());
    write(&args.output, lines);
}
