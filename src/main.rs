use clap::Clap;

use derse::Args;
use std::process;

fn main() {
    let args: Args = Args::parse();
    if let Err(e) = derse::run(args) {
        println!("Stopping with error: {}", e);
        process::exit(1);
    }
    process::exit(0);
}
