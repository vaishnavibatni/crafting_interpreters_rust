use clap::{arg, command, Parser};
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Parser)]
#[command(version = "1.0", about = "Jlox Interpreter ", long_about = None)]
struct Jlox {
    /// This is the name of the script we want to run
    script_name: String,
    /// In case we want to get verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn run_file(script_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(script_name)?;
    let reader = io::BufReader::new(file);
    let mut count = 1;
    for line in reader.lines() {
        println!("Line {}: {}", count, line?);
        count+=1
    }
    Ok(())
}
fn main() {
    let args = Jlox::parse();
    println!("Hello, world!");
    if args.verbose {
        println!("Verbose mode ON");
    }
    println!("Welcome to this script name: {0}", args.script_name);
    let _file_contents = run_file(&args.script_name);
}
