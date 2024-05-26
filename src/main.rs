use clap::Parser;

#[derive(Parser)]
struct CliArgs {
    path: std::path::PathBuf,
}

fn main() {
    let args = CliArgs::parse();

    println!("{:?}", args.path)
}
