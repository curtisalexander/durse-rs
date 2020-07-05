use clap::Clap;
// use std::fs;
use std::path::Path;

/// Get directory metadata
#[derive(Clap)]
// #[clap(version = "0.1.0", author = "Curtis Alexander <calex@calex.org>")]
struct Args {
    /// File for which to acquire metadata
    #[clap(long, short)]
    file: String,
}

fn main() -> std::io::Result<()> {
    let opts: Args = Args::parse();

    println!("Value for file: {}", opts.file);
    let path = Path::new(&opts.file);
    let md = path.metadata()?;
    // fs::metadata("derse.txt")?;

    println!("file type: {:?}", md.file_type());
    println!("is directory?: {:?}", md.is_dir());
    println!("is file?: {:?}", md.is_file());
    println!("len: {:?}", md.len());
    println!("persmissions: {:?}", md.permissions());
    // Unix => mtime field of stat
    // Windows => ftLastWriteTime field
    if let Ok(time) = md.modified() {
        println!("modified time{:?}", time);
    } else {
        println!("Not supported on this platform");
    }
    // Unix => atime field of stat
    // Windows => ftAccessTime field
    if let Ok(time) = md.accessed() {
        println!("accessed time{:?}", time);
    } else {
        println!("Not supported on this platform");
    }
    // Linux => btime field of statx
    // Unix => birthtime field of stat
    // Windows => ftCreationTime
    if let Ok(time) = md.created() {
        println!("created time{:?}", time);
    } else {
        println!("Not supported on this platform");
    }
    Ok(())
}
