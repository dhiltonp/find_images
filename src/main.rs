use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use std::env::current_dir;
use std::fs::{canonicalize, File};
use std::io::{LineWriter, Write};
use std::process::exit;
use structopt::StructOpt;

/// Returns true if `file` is a .jpg, .jpeg, .png, .bmp,
///  .webp, .gif or .tiff,... plus a bunch of raw file
///  extensions, handling case sensitivity.
fn is_image(file: &Path) -> bool {
    let image_types = [
        "jpg", "jpeg", "png", "bmp", "webp", "gif", "tiff", "pef", "dng", "crw", "nef", "cr2",
        "mrw", "rw2", "orf", "x3f", "arw", "kdc", "nrw", "dcr", "sr2", "raf",
    ];

    if let Some(extension) = file.extension() {
        let extension = extension.to_ascii_lowercase();
        for t in image_types.iter() {
            if extension == OsStr::new(t) {
                return true;
            }
        }
    }

    false
}

#[test]
fn test_is_image() {
    assert_eq!(is_image(&PathBuf::from("foo")), false);
    assert_eq!(is_image(&PathBuf::from("jpg")), false);
    assert_eq!(is_image(&PathBuf::from("blah.jpg")), true);
    assert_eq!(is_image(&PathBuf::from("blah.JPG")), true);
}

fn images(path: &Path) -> Vec<PathBuf> {
    let mut images = Vec::new();
    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() && is_image(&entry.path()) {
                    images.push(entry.path());
                }
            }
        }
    }
    images
}

fn subdirs(path: &Path) -> Vec<PathBuf> {
    let mut subdirs = Vec::new();
    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    subdirs.push(entry.path());
                }
            }
        }
    }
    subdirs
}

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long, default_value = "images.txt")]
    output: String,

    #[structopt(parse(from_os_str))]
    dirs: Vec<PathBuf>,
    // #[structopt(default_value = "32", long)]
    // images_per_dir: usize,
    // todo: add recursive mode
}

fn process_dir<W: Write>(dir: &Path, file: &mut LineWriter<W>) {
    println!("finding images in {:?}", &dir);
    for image in images(&dir) {
        file.write_all(&image.as_os_str().to_string_lossy().as_bytes())
            .expect("unable to write file, aborting!");
        file.write_all(b"\r\n")
            .expect("unable to write file, aborting!");
    }
    for subdir in subdirs(&dir) {
        process_dir(&subdir, file);
    }
}

fn main() {
    let args = Cli::from_args();
    if let Ok(file) = File::create(&args.output) {
        if let Ok(current_dir) = current_dir() {
            eprintln!("writing output to {:?} in {:?}", &args.output, current_dir);
        }
        let mut file = LineWriter::new(file);
        for dir in args.dirs {
            if let Ok(dir) = canonicalize(&dir) {
                process_dir(&dir, &mut file);
            } else {
                eprintln!("{:?} does not exist?", &dir);
            }
        }
    } else {
        if let Ok(current_dir) = current_dir() {
            eprintln!("unable to create {:?} in {:?}", &args.output, current_dir);
        }
        exit(1);
    }
}
