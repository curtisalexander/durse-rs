use std::error::Error;
use std::fs;
use std::io;
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

    // MVP
    // - Name
    // - Size
    // let r: Record = get_metadata(&args.path)?;
    // write_csv_file(&args.csv, r)?;
    if args.path.is_dir() {
        walk_dir(&args.path)?;
    } else {
        let r: Record = get_metadata(&args.path)?;
        write_csv_file(&args.csv, r)?;
    }

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

fn get_metadata(path: &PathBuf) -> Result<Record, Box<dyn Error>> {
    let md = path.metadata()?;
    let name: &str = path.to_str().unwrap_or_default();
    let size = md.len();

    Ok(Record { name, size })
}

fn walk_dir(dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        let r: Record = get_metadata(&path)?;
        write_csv_stdout(r)?;
    }
    Ok(())
}

fn write_csv_file(path: &PathBuf, r: Record) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::WriterBuilder::new()
        .quote_style(csv::QuoteStyle::Always)
        .from_path(path)?;

    wtr.serialize(r)?;
    wtr.flush()?;
    Ok(())
}

fn write_csv_stdout(r: Record) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::WriterBuilder::new()
        .quote_style(csv::QuoteStyle::Always)
        .from_writer(io::stdout());

    wtr.serialize(r)?;
    wtr.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_metadata() {
        let path = PathBuf::from("derse.txt");
        let r: Record = super::get_metadata(&path).unwrap();
        assert_eq!(r.name, "derse.txt");
        assert_eq!(r.size, 89);
    }
}
