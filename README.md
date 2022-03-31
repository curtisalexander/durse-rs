# durse-rs

<!--[![Build Status](https://dev.azure.com/curtisalexander/durse-rs/_apis/build/status/curtisalexander.durse-rs%20(9)?branchName=master)](https://dev.azure.com/curtisalexander/durse-rs/_build/latest?definitionId=17&branchName=master) -->

```
(d)irectory + rec(urse) = durse
```

Recursively acquire file metatdata

## Use
Generated from [clap](https://crates.io/crates/clap) via `durse --help`

```
durse
(d)irectory + rec(urse) => recursively acquire file metadata within a directory

USAGE:
    durse.exe [OPTIONS] [PATH]

ARGS:
    <PATH>    Directory to begin recursive walk, begin in current directory if no value provided

OPTIONS:
    -f, --file-name <FILE_NAME>    Path to file to write results, writes to stdout if not present
    -h, --help                     Print help information
    -o, --out-type <OUT_TYPE>      Output type, defaults to csv if not provided [default: csv]
                                   [possible values: csv, ndjson]
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

## Github Actions
Below is the rough `git tag` dance to delete and/or add tags to [trigger Github Actions](https://github.com/curtisalexander/readstat-rs/blob/main/.github/workflows/main.yml#L7-L10).

```sh
# delete local tag
git tag --delete v0.1.0

# delete remote tag
git push origin --delete v0.1.0

# add and commit local changes
git add .
git commit -m "commit msg"

# push local changes to remote
git push

# add local tag
git tag -a v0.1.0 -m "v0.1.0"

# push local tag to remote
git push origin --tags
```