use clap::Clap;

use std::path::PathBuf;

/// Get directory metadata
#[derive(Clap)]
// #[clap(version = "0.1.0", author = "Curtis Alexander <calex@calex.org>")]
pub struct Args {
    /// Path to acquire metadata
    #[clap(long, short)]
    pub path: PathBuf,
}

pub fn run(args: Args) -> std::io::Result<()> {
    // println!("Value for file: {}", args.file);

    // let path = Path::new(&args.file);
    let md = args.path.metadata()?;
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

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn clap_parsing() {
        assert_eq!(1, 1);
    }
}
