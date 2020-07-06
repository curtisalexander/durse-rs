# derse-rs

```
(d)irectory + t(erse) = derse
```

Get directory metadata

## Use
Generated from [clap](https://clap.rs/).

```
derse
Get directory metadata

USAGE:
    derse --path <path> --csv <csv>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --csv <csv>      Path to csv file to write results
    -p, --path <path>    Path to acquire metadata
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
