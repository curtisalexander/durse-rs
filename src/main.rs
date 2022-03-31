use clap::Parser;
use durse::Args;
use std::process;

fn main() {
    let args = Args::parse();
    if let Err(e) = durse::run(args) {
        println!("Stopping with error: {}", e);
        process::exit(1);
    }
    process::exit(0);
}
