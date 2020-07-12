# durse-rs

```
(d)irectory + rec(urse) = durse
```

Recursively acquire file metatdata

## Use
Generated from [structopt](https://crates.io/crates/structopt)

```
durse 0.1.0
(d)irectory + rec(urse) => recursively acquire file metadata within a directory

USAGE:
    durse [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file-name <file-name>    Path to file to write results, writes to stdout if not present
    -o, --out-type <out-type>      Output type, defaults to csv if not provided [default: csv]  [possible values: csv,
                                   json]
    -p, --path <path>              Directory to begin recursive walk, begin in current directory if no value provided
```

## Inspiration

Would like to adapt the following PowerShell one-liner.

```powershell
$SearchDir = "/some/path"
$OutCSV = "results.csv"

Get-ChildItem -Path $SearchDir -Recurse -File |
Select-Object FullName, Name, Basename, Extension, DirectoryName, CreationTime, LastAccessTime, LastWriteTime, `
@{Name = 'Owner' ; Expression = { $(Get-Acl $_).Owner } }, @{Name = 'Size KB'; Expression = { $_.Length / 1KB } } |
Export-Csv -Path $OutCSV -NoTypeInformation
```

> **NOTE:** The [Get-Acl]() cmdlet only works on Windows.  For details one getting file permissions on Linux or macOS, see [Working with Linux Permissions in PowerShell 7](https://petri.com/working-with-linux-permissions-in-powershell-7).
