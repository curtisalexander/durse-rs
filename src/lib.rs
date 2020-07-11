use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;

use serde::Serialize;
use structopt::clap::arg_enum;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Recursively acquire file metadata")]
pub struct Args {
    /// Directory to begin recursive walk, begin in current directory if no value provided
    #[structopt(long, short)]
    pub path: Option<PathBuf>,
    /// Output type, stdout if not present
    #[structopt(long, short, possible_values = &OutType::variants(), case_insensitive = true)]
    pub out_type: Option<OutType>,
    /// Path to csv or json file to write results
    #[structopt(long, short, required_ifs(&[("out_type", "csv"), ("out_type", "json")]), parse(from_os_str))]
    pub file_name: PathBuf,
}

arg_enum! {
    #[derive(Debug)]
    pub enum OutType {
        Csv,
        Json
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
    out_type: Option<OutType>,
    file_name: PathBuf,
    set: Vec<Record>,
}

impl RecordSet {
    fn new(out_type: Option<OutType>, file_name: PathBuf) -> Self {
        Self {
            out_type: out_type,
            file_name: file_name,
            set: Vec::with_capacity(10),
        }
    }

    fn write(&self) -> Result<(), Box<dyn Error>> {
        match self.out_type {
            Some(OutType::Csv) => {
                let mut wtr = csv::WriterBuilder::new()
                    .quote_style(csv::QuoteStyle::Always)
                    .from_path(&self.file_name)?;

                for r in &self.set {
                    wtr.serialize(r)?;
                }
                wtr.flush()?;
                Ok(())
            }
            Some(OutType::Json) => {
                return Err(From::from(
                    "Sorry, wrriting to JSON not yet implemented!  :(",
                ))
            }
            None => {
                let mut wtr = csv::WriterBuilder::new()
                    .quote_style(csv::QuoteStyle::Always)
                    .from_writer(io::stdout());

                for r in &self.set {
                    wtr.serialize(r)?;
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
        walk_dir(&path, args.out_type, args.file_name)?;
    } else {
        let r: Record = get_metadata(&path)?;
        write_csv_file(&args.file_name, r)?;
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
    out_type: Option<OutType>,
    file_name: PathBuf,
) -> Result<(), Box<dyn Error>> {
    let mut records = RecordSet::new(out_type, file_name);

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        let r = get_metadata(&path).unwrap();

        records.set.push(r);
    }

    records.write()?;
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
