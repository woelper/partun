# partun

**par**tial **un**archival tool.

[![Rust](https://github.com/woelper/partun/actions/workflows/rust.yml/badge.svg)](https://github.com/woelper/partun/actions/workflows/rust.yml)

This is a very specialized command line utility which allows partial decompression of archives, for example in memory or diskspace constrained systems.
Partun can also be used where available methods (for example in 32 bit environments and very large zip files) fail to extract files.

```
Partun 
Extracts zip files partially

USAGE:
    partun [OPTIONS] <ZIP>

ARGS:
    <ZIP>    Sets the input archive

OPTIONS:
        --debug                       Toggle debug output
    -e, --exclude <exclude>           Do not extract file containing this string. Use commas for
                                      multiple exclusions.
        --ext <ext>                   Only extract files with this extension (e.g. gif). Use commas
                                      for multiple exclusions.
    -f, --filter <filter>             Only extract file containing this string
    -h, --help                        Print help information
    -i, --ignorepath                  Extract all files to current dir, ignoring all paths
        --include-archive-name        When listing, include the archive name in path
    -l, --list                        List files instead of extracting, one per line. Filtes apply.
    -o, --output <output>             extract files to this location
    -r, --random                      Extract only a random file. This can be combined with the
                                      filter flag.
        --rename <rename>             Rename EVERY file to this string. Useful in scripts with the
                                      random option
        --skip-duplicate-filenames    Do not extract duplicate file names

```
