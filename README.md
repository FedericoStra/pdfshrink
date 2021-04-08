# pdfshrink

Shrink PDF files using Ghostscript.

This is a (WIP) reimplementation of [PDF-Slim](https://github.com/FedericoStra/pdfslim/) in Rust.

`pdfshrink` takes a number of PDF files and tries to optimize them through a suitable call to `ghostscript`.
It is specifically fitted to reduce the size of scanned papers, containing images with humongous resolution.

## Installation

To install `pdfshrink` for the first time or upgrade it to the latest version, run the following command at your shell:

```bash
cargo +nightly install --features build-binary pdfshrink
```

## Help message

```
$ pdfshrink --help
pdfshrink 0.1.7
Federico Stra <stra.federico@gmail.com>
Shrink PDF files using Ghostscript

USAGE:
    pdfshrink [OPTIONS] <INPUT>...

OPTIONS:
    -n, --dry-run            Do not actually run the commands, just show them
    -h, --help               Print help information
    -i, --inplace            Replace the original file
    -r, --rename             Save the output to a renamed file: *.pdf -> *.shrunk.pdf (defaut)
    -d, --subdir <SUBDIR>    Save the output in a subdirectory
    -V, --version            Print version information
    -v, --verbose            Increase the level of verbosity

ARGS:
    <INPUT>...    Input PDF files to shrink

The options --inplace, --rename and --subdir are mutually exclusive.
```
