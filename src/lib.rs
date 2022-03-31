use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::{File, Metadata};
use std::io;
use std::io::{BufWriter, Write};
use std::os::windows::prelude::OsStrExt;
// use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use std::string::FromUtf16Error;

use clap::{ArgEnum, Parser};
use chrono::{DateTime, Local};
use jwalk::WalkDir;
use path_clean::PathClean;
use serde::Serialize;
use windows::{
    core::{PCWSTR, PWSTR},
    Win32::{
        Foundation::{GetLastError, ERROR_SUCCESS, HANDLE, PSID},
        Security::{
            Authorization::{GetSecurityInfo, SE_FILE_OBJECT},
            LookupAccountSidW, SidTypeUnknown, OWNER_SECURITY_INFORMATION, SECURITY_DESCRIPTOR,
            SID_NAME_USE,
        },
        Storage::FileSystem::{
            CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_GENERIC_READ, FILE_SHARE_READ, OPEN_EXISTING,
        },
    },
};

#[derive(Parser, Debug)]
#[clap(
    about = "(d)irectory + rec(urse) => recursively acquire file metadata within a directory",
    long_about = None
)]
pub struct Args {
    /// Directory to begin recursive walk, begin in current directory if no value provided
    #[clap()]
    pub path: Option<PathBuf>,
    /// Path to file to where metadata will be written{n}Results written to stdout if not provided
    #[clap(long, short, parse(from_os_str))]
    pub file_name: Option<PathBuf>,
    /// Output file type
    #[clap(arg_enum, long, short, default_value = "csv", ignore_case = true)]
    pub out_type: OutType,
}

#[derive(ArgEnum, Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum OutType {
    csv,
    ndjson
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Record<'a> {
    pub run_date: &'a str,
    pub full_name: String,
    pub name: String,
    pub base_name: String,
    pub is_directory: bool,
    pub extension: String,
    pub directory_name: String,
    pub creation_time: DateTime<Local>,
    pub last_access_time: DateTime<Local>,
    pub last_modified_time: DateTime<Local>,
    pub owner: String,
    pub size: u64,
    pub size_kb: f64,
    pub size_mb: f64,
    pub size_gb: f64,
    pub size_tb: f64,
}

#[derive(Debug)]
struct RecordSet<'a> {
    file_name: Option<PathBuf>,
    out_type: OutType,
    set: Vec<Record<'a>>,
}

impl RecordSet<'_> {
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

#[derive(Debug)]
struct WideString(Vec<u16>);

impl WideString {
    fn as_const_ptr(&self) -> *const u16 {
        let s_ref: &Vec<u16> = &self.0.as_ref();
        s_ref.as_ptr() as *const u16
    }

    fn as_ptr(&self) -> *mut u16 {
        let s_ref: &Vec<u16> = &self.0.as_ref();
        s_ref.as_ptr() as *mut u16
    }

    fn from_os_str(s: &OsStr) -> Self {
        Self(s.encode_wide().chain(std::iter::once(0)).collect())
    }

    #[allow(dead_code)]
    fn from_str(s: &str) -> Self {
        Self(s.encode_utf16().chain(std::iter::once(0)).collect())
    }

    fn new(capacity: usize) -> Self {
        let mut v: Vec<u16> = Vec::default();
        v.resize(capacity, 0);
        Self(v)
    }

    #[allow(dead_code)]
    fn to_string(&self) -> Result<String, FromUtf16Error> {
        let v = &self.0;
        String::from_utf16(&v[..(v.len() - 1)]) // remove trailing null
    }
}

#[cfg(target_os = "windows")]
fn get_file_owner(path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let path_as_wstring = WideString::from_os_str(path.as_os_str());
    let path_as_wstring_ptr = path_as_wstring.as_const_ptr();
    let path_as_pcwstr = PCWSTR(path_as_wstring_ptr);

    // File handle
    let handle: HANDLE = unsafe {
        CreateFileW(
            path_as_pcwstr,
            FILE_GENERIC_READ,
            FILE_SHARE_READ,
            std::ptr::null_mut(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            None,
        )
    };

    if let Err(e) = handle.ok() {
        panic!("Error with {:#?}: {:#?}", path_as_pcwstr, e);
    }

    // Security Info
    let mut psidowner = PSID::default();
    let mut sd: *mut SECURITY_DESCRIPTOR =
        &mut SECURITY_DESCRIPTOR::default() as *mut SECURITY_DESCRIPTOR;

    let gsi_rc = unsafe {
        GetSecurityInfo(
            handle,
            SE_FILE_OBJECT,
            OWNER_SECURITY_INFORMATION.0,
            &mut psidowner,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut sd,
        )
    };

    if gsi_rc != ERROR_SUCCESS.0 {
        let last_error = unsafe { GetLastError() };
        panic!("Error code is {:#?}", last_error);
    }

    // Lookup Account Sid
    let mut name_size = 0 as u32;
    let mut domain_size = 0 as u32;

    let name_as_wstring = WideString::new(name_size as usize);
    let name_as_wstring_ptr = name_as_wstring.as_ptr();
    let name_as_pwstr = PWSTR(name_as_wstring_ptr);

    let domain_as_wstring = WideString::new(domain_size as usize);
    let domain_as_wstring_ptr = domain_as_wstring.as_ptr();
    let domain_as_pwstr = PWSTR(domain_as_wstring_ptr);

    let euse = &mut SidTypeUnknown.to_owned() as *mut SID_NAME_USE;

    // Call to get size of name_size and domain_size
    let las_rc = unsafe {
        LookupAccountSidW(
            None,
            psidowner,
            name_as_pwstr,
            &mut name_size,
            domain_as_pwstr,
            &mut domain_size,
            euse,
        )
    };

    if las_rc.0 != 0 {
        panic!("Expecting an error when calling LookupAccountSidW initially");
    }

    // Call again, this time with appropriately sized buffers
    let name_as_wstring = WideString::new(name_size as usize);
    let name_as_wstring_ptr = name_as_wstring.as_ptr();
    let name_as_pwstr = PWSTR(name_as_wstring_ptr);

    let domain_as_wstring = WideString::new(domain_size as usize);
    let domain_as_wstring_ptr = domain_as_wstring.as_ptr();
    let domain_as_pwstr = PWSTR(domain_as_wstring_ptr);

    let las_rc = unsafe {
        LookupAccountSidW(
            None,
            psidowner,
            name_as_pwstr,
            &mut name_size,
            domain_as_pwstr,
            &mut domain_size,
            euse,
        )
    };

    if las_rc.0 == 0 {
        let last_error = unsafe { GetLastError() };
        panic!("Error code is {:#?}", last_error);
    }
    
    let owner = name_as_wstring.to_string()?;

    Ok(owner)
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    // Implement the following:
    //   - RunDate
    //   - FullName
    //   - Name
    //   - IsDirectory
    //   - Basename
    //   - Extension
    //   - DirectoryName
    //   - CreationTime
    //   - LastAccessTime
    //   - LastWriteTime
    //   - Owner
    //   - Size B
    //   - Size KB (distinguish kilobytes from kibibytes)
    //   - Size MB (distinguish megaytes from mebibytes)
    //   - Size GB (distinguish gigabytes from gibibytes)
    //   - Size TB (distinguish terabytes from tebibytes)

    // Process args
    let path = &args.path.unwrap_or(std::env::current_dir()?);

    // Validate file_name
    match file_name_valid(&args.file_name).unwrap() {
        (true, _) => (),
        (false, parent) => {
            return Err(From::from(format!(
            "The parent directory of the value of the parameter --file-name ({}) does not exist",
            parent
        )))
        }
    };

    if path.is_dir() {
        walk_dir(&path, args.file_name, args.out_type)?;
    } else {
        return Err(From::from(
            "The provided value of the argument <path> was not a directory",
        ));
    }

    Ok(())
}

fn file_name_valid(f: &Option<PathBuf>) -> Result<(bool, String), Box<dyn Error>> {
    let result = match f {
        None => (true, String::from("")),
        Some(p) => {
            // https://stackoverflow.com/a/54817755
            let abs_path = if p.is_absolute() {
                p.to_path_buf()
            } else {
                env::current_dir()?.join(p)
            }
            .clean();

            abs_path.parent().map_or_else(
                || (false, String::from("no_parent")),
                |parent| (parent.exists(), parent.to_string_lossy().into_owned()),
            )
        }
    };

    Ok(result)
}

fn get_metadata<'a>(
    path: &PathBuf,
    md: &Metadata,
    run_date: &'a str,
) -> Result<Record<'a>, Box<dyn Error>> {
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
    let is_directory = path.is_dir();
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
    //let owner = String::from("");
    let owner = if cfg!(windows) {
        get_file_owner(&path)?
    } else {
        String::from("")
    };
    // let owner = std::fs::metadata(path)?.uid();
    let size = md.len();
    // kibibyes
    let size_kb = (md.len() as f64) / 1024_f64.powi(1);
    // mebibytes
    let size_mb = (md.len() as f64) / 1024_f64.powi(2);
    // gibibytes
    let size_gb = (md.len() as f64) / 1024_f64.powi(3);
    // tebibytes
    let size_tb = (md.len() as f64) / 1024_f64.powi(4);

    Ok(Record {
        run_date,
        full_name,
        name,
        base_name,
        is_directory,
        extension,
        directory_name,
        creation_time,
        last_access_time,
        last_modified_time,
        owner,
        size,
        size_kb,
        size_mb,
        size_gb,
        size_tb,
    })
}

fn walk_dir(
    dir: &PathBuf,
    file_name: Option<PathBuf>,
    out_type: OutType,
) -> Result<(), Box<dyn Error>> {
    let mut records = RecordSet::new(file_name, out_type);

    let run_date = Local::now().to_string();

    for entry in WalkDir::new(dir).skip_hidden(false) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let md = entry.metadata()?;
            let r = get_metadata(&path, &md, &run_date)?;
            records.set.push(r);
        }
    }

    records.write()?;
    Ok(())
}
