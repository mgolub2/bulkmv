use std::path::PathBuf;

use clap::Parser;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use retry::{delay::Fixed, retry};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The path to the source directory.
    src: Option<PathBuf>,
    /// The path to the destination directory.
    dest: Option<PathBuf>,
}

fn build_src_file_list(src: &PathBuf) -> Vec<PathBuf> {
    let mut file_list = Vec::new();
    // recursively iterate through the source directory, adding all files to the file list:
    for entry in WalkDir::new(src)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            file_list.push(entry.path().to_path_buf());
        }
    }
    file_list
}

fn main() {
    let cli = Cli::parse();
    let src = cli.src.expect("msg: src directory not specified");
    let dest = cli.dest.expect("msg: dest directory not specified");
    let file_list = build_src_file_list(&src);
    file_list
        .par_iter()
        .progress_count(file_list.len() as u64)
        .for_each(|file| {
            let mut dest_file = dest.clone();
            dest_file.push(file.strip_prefix(&src).unwrap());
            let dest_file_path = dest_file.as_path();
            // create the destination directory if it doesn't exist:
            if let Some(parent) = dest_file.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent).expect("msg: failed to create directory");
                }
            }
            //retries forever until the file is copied:
            retry(Fixed::from_millis(100), || {
                std::fs::copy(file, dest_file_path)
            })
            .expect("msg: failed to copy file");
        });
}
