use clap::Parser;
use pom_says_fix::{CliArgs, FileContents};

fn main() {
    let args = CliArgs::parse();
    let contents = std::fs::read_to_string(&args.path).expect("Valid file path.");
    let extension = args
        .path
        .extension()
        .expect("File with extension.")
        .to_str()
        .expect("Valid unicode extension")
        .to_string();

    let file_contents = FileContents::new(contents, extension);
    println!(
        "File Contents:\n{}\nFile Extension:\n{}",
        file_contents.contents, file_contents.extension
    )
}
