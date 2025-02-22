use clap::{arg, command, Parser};

#[derive(Parser)]
#[command(version = "1.0", about = "A simple CLI tool in Rust", long_about = None)]
struct Jlox {
    input: String,
    #[arg(short, long)]
    verbose: bool,
}
fn main() {
    let args = Jlox::parse();
    println!("Hello, world!");
    if args.verbose {
        println!("Verbose mode");
    }
    println!("Welcome to this input: {0}", args.input);
}
