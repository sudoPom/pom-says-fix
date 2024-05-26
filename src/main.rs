use clap::Parser;

#[derive(Parser)]
struct CliArgs {
    path: std::path::PathBuf,
}

fn main() {
    let args = CliArgs::parse();
    let content = std::fs::read_to_string(args.path).expect("Invalid file path given.");
    println!("{}", content);
}
