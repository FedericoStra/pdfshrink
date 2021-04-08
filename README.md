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
pdfshrink 0.1.0
Federico Stra <stra.federico@gmail.com>
Shrink PDF files using Ghostscript

USAGE:
    pdfshrink [FLAGS] [OPTIONS] <INPUT>...

FLAGS:
        --debug      debug the command line
    -n, --dry-run    dry run
    -h, --help       print help information
    -i, --inplace    replace the original file
    -r, --rename     save the output in a renamed file: .pdf -> .slim.pdf
    -V, --version    Prints version information
    -v, --verbose    set the level of verbosity

OPTIONS:
    -d, --subdir <SUBDIR>    save the output in a subdirectory

ARGS:
    <INPUT>...    input PDF to shrink

The options --inplace, --rename and --subdir are mutually exclusive.
```
