use std::error::Error;
use std::path::PathBuf;

use clap::Clap;
use serde::Serialize;

/// Get directory metadata
#[derive(Clap)]
// #[clap(version = "0.1.0", author = "Curtis Alexander <calex@calex.org>")]
pub struct Args {
    /// Path to acquire metadata
    #[clap(long, short)]
    pub path: PathBuf,
    /// Path to csv file to write
    #[clap(long, short, parse(from_os_str))]
    pub csv: PathBuf,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Record<'a> {
    pub name: &'a str,
    pub size: u64,
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    // pub fn run(args: Args) -> std::io::Result<()> {
    // Implement the following:
    // - FullName
    // - Name
    // - Basename
    // - Extension
    // - DirectoryName
    // - CreationTime
    // - LastAccessTime
    // - LastWriteTime
    // - Owner
    // - Size B
    // - Size KB (distinguish Kilobyes from Kibibytes)
    // - Size MB
    // - Size GB

    // let mut wtr = csv::Writer::from_path(args.csv)?;
    let mut wtr = csv::WriterBuilder::new()
        .quote_style(csv::QuoteStyle::Always)
        .from_path(args.csv)?;

    // MVP
    // - Name
    // - Size
    let md = args.path.metadata()?;
    let name = args.path.to_str().unwrap_or_default();
    let size = md.len();

    let r = Record { name, size };
    // println!("Record: {:?}", r);

    wtr.serialize(r)?;
    wtr.flush()?;
    Ok(())
    /*
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
    */
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn clap_parsing() {
        assert_eq!(1, 1);
    }
}
