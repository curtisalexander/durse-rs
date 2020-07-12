use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use serde::Serialize;
use structopt::clap::arg_enum;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    about = "(d)irectory + rec(urse) => recursively acquire file metadata within a directory"
)]
pub struct Args {
    /// Directory to begin recursive walk, begin in current directory if no value provided
    #[structopt(long, short)]
    pub path: Option<PathBuf>,
    /// Path to file to write results, writes to stdout if not present
    #[structopt(long, short, parse(from_os_str))]
    pub file_name: Option<PathBuf>,
    /// Output type, defaults to csv if not provided
    #[structopt(long, short, default_value = "csv", possible_values = &OutType::variants(), case_insensitive = true)]
    pub out_type: OutType,
}

arg_enum! {
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    pub enum OutType {
        csv,
        json
    }
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Record {
    pub name: String,
    pub size: u64,
}

#[derive(Debug)]
struct RecordSet {
    file_name: Option<PathBuf>,
    out_type: OutType,
    set: Vec<Record>,
}

impl RecordSet {
    fn new(file_name: Option<PathBuf>, out_type: OutType) -> Self {
        Self {
            file_name: file_name,
            out_type: out_type,
            set: Vec::with_capacity(10),
        }
    }

    fn write(&self) -> Result<(), Box<dyn Error>> {
        match (&self.file_name, &self.out_type) {
            (Some(file_name), OutType::csv) => {
                let mut wtr = csv::WriterBuilder::new()
                    .quote_style(csv::QuoteStyle::Always)
                    .from_path(file_name)?;

                for r in &self.set {
                    wtr.serialize(r)?;
                }
                wtr.flush()?;
                Ok(())
            }
            (None, OutType::csv) => {
                let mut wtr = csv::WriterBuilder::new()
                    .quote_style(csv::QuoteStyle::Always)
                    .from_writer(io::stdout());

                for r in &self.set {
                    wtr.serialize(r)?;
                }
                Ok(())
            }
            (Some(file_name), OutType::json) => {
                let f = File::create(file_name)?;
                let mut wtr = BufWriter::new(f);

                for r in &self.set {
                    serde_json::to_writer(&mut wtr, r)?;
                    wtr.write_all(b"\n")?;
                }
                wtr.flush()?;
                Ok(())
            }
            (None, OutType::json) => {
                let mut wtr = BufWriter::new(io::stdout());

                for r in &self.set {
                    serde_json::to_writer(&mut wtr, r)?;
                    wtr.write_all(b"\n")?;
                }
                wtr.flush()?;
                Ok(())
            }
        }
    }
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
    // - Size KB (distinguish Kilobytes from Kibibytes)
    // - Size MB
    // - Size GB

    // Process args
    let path = &args.path.unwrap_or(std::env::current_dir()?);

    // MVP
    // - Name
    // - Size
    // let r: Record = get_metadata(&args.path)?;
    // write_csv_file(&args.csv, r)?;
    if path.is_dir() {
        walk_dir(&path, args.file_name, args.out_type)?;
    } else {
        return Err(From::from("The provided value of path was not a directory"));
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
    let name = path.to_string_lossy().into_owned();
    // let name: &str = path.to_str().unwrap_or_default();
    let size = md.len();

    Ok(Record { name, size })
}

fn walk_dir(
    dir: &PathBuf,
    file_name: Option<PathBuf>,
    out_type: OutType,
) -> Result<(), Box<dyn Error>> {
    let mut records = RecordSet::new(file_name, out_type);

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        let r = get_metadata(&path).unwrap();

        records.set.push(r);
    }

    records.write()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_metadata() {
        let path = PathBuf::from("durse.txt");
        let r: Record = super::get_metadata(&path).unwrap();
        assert_eq!(r.name, "durse.txt");
        assert_eq!(r.size, 89);
    }
}
