# durse-rs

<!--[![Build Status](https://dev.azure.com/curtisalexander/durse-rs/_apis/build/status/curtisalexander.durse-rs%20(9)?branchName=master)](https://dev.azure.com/curtisalexander/durse-rs/_build/latest?definitionId=17&branchName=master) -->

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
    durse [OPTIONS] [path]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file-name <file-name>    Path to file to write results, writes to stdout if not present
    -o, --out-type <out-type>      Output type, defaults to csv if not provided [default: csv]  [possible values: csv,
                                   ndjson]

ARGS:
    <path>    Directory to begin recursive walk, begin in current directory if no value provided
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

## Azure Pipelines

To trigger Azure Pipelines, include a tag.

```
# delete current tag
git tag -d v0.1.0

# delete from origin
git push --delete origin v0.1.0

# create tag
git tag v0.1.0

# push to origin
git push --tags origin
```