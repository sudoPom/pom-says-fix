use core::panic;

use clap::Parser;

#[derive(Parser)]
pub struct CliArgs {
    pub path: std::path::PathBuf,
}

pub struct FileContents {
    pub contents: String,
    pub extension: String,
}

impl FileContents {
    pub fn new(contents: String, extension: String) -> FileContents {
        return FileContents {
            contents,
            extension,
        };
    }
}

pub fn run(file_contents: FileContents) {
    let cfg = gen_cfg(file_contents);
}

pub fn gen_cfg(file_contents: FileContents) {
    match file_contents.extension.as_str() {
        "rs" => gen_rust_cfg(file_contents.contents),
        _ => panic!(),
    }
}

pub fn gen_rust_cfg(source_file: String) {}
