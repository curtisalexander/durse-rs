use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use chrono::{DateTime, Local};
use jwalk::WalkDir;
use serde::Serialize;
use structopt::clap::arg_enum;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    about = "(d)irectory + rec(urse) => recursively acquire file metadata within a directory"
)]
pub struct Args {
    /// Directory to begin recursive walk, begin in current directory if no value provided
    #[structopt()]
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
        ndjson
    }
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Record {
    pub full_name: String,
    pub name: String,
    pub base_name: String,
    pub extension: String,
    pub directory_name: String,
    pub creation_time: DateTime<Local>,
    pub last_access_time: DateTime<Local>,
    pub last_modified_time: DateTime<Local>,
    pub size: u64,
    pub size_kb: f64,
    pub size_mb: f64,
    pub size_gb: f64,
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
            file_name,
            out_type,
            set: Vec::with_capacity(20),
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
            (Some(file_name), OutType::ndjson) => {
                let f = File::create(file_name)?;
                let mut wtr = BufWriter::new(f);

                for r in &self.set {
                    serde_json::to_writer(&mut wtr, r)?;
                    wtr.write_all(b"\n")?;
                }
                wtr.flush()?;
                Ok(())
            }
            (None, OutType::ndjson) => {
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
    //   - FullName
    //   - Name
    //   - Basename
    //   - Extension
    //   - DirectoryName
    //   - CreationTime
    //   - LastAccessTime
    //   - LastWriteTime
    //   - ** Owner **
    //   - Size B
    //   - Size KB (distinguish Kilobytes from Kibibytes)
    //   - Size MB
    //   - Size GB

    // Process args
    let path = &args.path.unwrap_or(std::env::current_dir()?);

    // Validate file_name
    /*
    match file_name_valid(&args.file_name).unwrap() {
        (true, _) => (),
        (false, parent) => {
            return Err(From::from(format!(
            "The parent directory of the value of the parameter --file-name ({}) does not exist",
            parent
        )))
        }
    };
    */

    if path.is_dir() {
        walk_dir(&path, args.file_name, args.out_type)?;
    } else {
        return Err(From::from(
            "The provided value of the argument <path> was not a directory",
        ));
    }

    Ok(())

    /*
    println!("persmissions: {:?}", md.permissions());
    */
}

/*
fn file_name_valid(f: &Option<PathBuf>) -> Result<(bool, String), Box<dyn Error>> {
    let result = match f {
        None => (true, String::from("")),
        Some(p) => p.canonicalize()?.parent().map_or_else(
            || (false, String::from("no_parent")),
            |parent| (parent.exists(), parent.to_string_lossy().into_owned()),
        ),
    };
    Ok(result)
}
*/

fn get_metadata(path: &PathBuf) -> Result<Record, Box<dyn Error>> {
    let md = path.metadata()?;

    let full_name = path.to_string_lossy().into_owned();
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default()
        .to_owned();
    let base_name = path
        .file_stem()
        .and_then(|b| b.to_str())
        .unwrap_or_default()
        .to_owned();
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
        .to_owned();
    let directory_name = path
        .parent()
        .map(|d| d.to_string_lossy().into_owned())
        .unwrap_or_default();
    // Linux => btime field of statx
    // Unix => birthtime field of stat
    // Windows => ftCreationTime
    let creation_time: DateTime<Local> = DateTime::from(md.created()?);
    // Unix => atime field of stat
    // Windows => ftAccessTime field
    let last_access_time: DateTime<Local> = DateTime::from(md.accessed()?);
    // Unix => mtime field of stat
    // Windows => ftLastWriteTime field
    let last_modified_time: DateTime<Local> = DateTime::from(md.modified()?);
    let size = md.len();
    // kibibyes
    let size_kb = (md.len() as f64) / 1024_f64.powi(2);
    // mebibytes
    let size_mb = (md.len() as f64) / 1024_f64.powi(3);
    // gibibytes
    let size_gb = (md.len() as f64) / 1024_f64.powi(4);

    Ok(Record {
        full_name,
        name,
        base_name,
        extension,
        directory_name,
        creation_time,
        last_access_time,
        last_modified_time,
        size,
        size_kb,
        size_mb,
        size_gb,
    })
}

fn walk_dir(
    dir: &PathBuf,
    file_name: Option<PathBuf>,
    out_type: OutType,
) -> Result<(), Box<dyn Error>> {
    let mut records = RecordSet::new(file_name, out_type);

    for entry in WalkDir::new(dir).sort(true) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let r = get_metadata(&path)?;
            records.set.push(r);
        }
    }

    records.write()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn minimal() {
        assert_eq!(1, 1)
    }
}
