# partun

**par**tial **un**archival tool.

This is a very niche command line utility which allows partial decompression of archives, for example in memory or diskspace constrained systems.
Partun can also be used where available methods (for example in 32 bit environments and very large zip files) fail to extract files.

```
Partun 
Extracts zip files partially

USAGE:
    partun [FLAGS] [OPTIONS] <ZIP>

FLAGS:
    -h, --help          Prints help information
    -i, --ignorepath    Extract all files to current dir, ignoring all paths
    -r, --random        Extract only a random file. This can be combined with the filter flag.
    -V, --version       Prints version information

OPTIONS:
    -e, --exclude <exclude>    Do not extract file containing this string
    -f, --filter <filter>      Only extract file containing this string
        --rename <rename>      Rename EVERY file to this string. Useful in scripts with the random option

ARGS:
    <ZIP>    Sets the input file to use
```
