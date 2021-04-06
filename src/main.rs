#[macro_use]
extern crate clap;
use clap::{Arg, ArgGroup};

use pdfshrink::*;

fn main() {
    let app = app_from_crate!()
        .help_message("print help information")
        .after_help("The options --inplace, --rename and --subdir are mutually exclusive.")
        .arg(
            Arg::with_name("input")
                .multiple(true)
                .required(true)
                .value_name("INPUT")
                .help("input PDF to shrink"),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .multiple(true)
                .help("set the level of verbosity"),
        )
        .arg(
            Arg::with_name("inplace")
                .long("inplace")
                .short("i")
                .help("replace the original file"),
        )
        .arg(
            Arg::with_name("rename")
                .long("rename")
                .short("r")
                .help("save the output in a renamed file: .pdf -> .slim.pdf"),
        )
        .arg(
            Arg::with_name("subdir")
                .long("subdir")
                .short("d")
                .value_name("SUBDIR")
                .help("save the output in a subdirectory"),
        )
        .arg(
            Arg::with_name("debug")
                .long("debug")
                .help("debug the command line"),
        )
        .arg(
            Arg::with_name("dry-run")
                .long("dry-run")
                .short("n")
                .help("dry run"),
        )
        .group(ArgGroup::with_name("output").args(&["inplace", "rename", "subdir"]));

    let matches = app.get_matches();

    let debug = matches.is_present("debug");
    let dry_run = matches.is_present("dry-run");
    let verbose = matches.is_present("verbose");

    // BEGIN DEBUG
    if debug {
        eprintln!("{:#?}", matches);
        eprintln!("---");
        for arg in &["input", "output", "inplace", "rename", "subdir", "verbose"] {
            eprintln!(
                "{} ({} {}): {:?}",
                arg,
                matches.occurrences_of(arg),
                matches.is_present(arg),
                matches.values_of(arg)
            );
        }
        eprint!("output mode: ");
        if matches.is_present("inplace") {
            eprintln!("inplace");
        } else if matches.is_present("rename") {
            eprintln!("rename");
        } else if matches.is_present("subdir") {
            eprintln!(
                "subdir: {:?}",
                matches.value_of("subdir").unwrap_or("shrunk")
            );
        } else {
            eprintln!("no output mode specified");
        }
        eprintln!();
    }
    // END DEBUG

    for inpath in matches.values_of("input").expect("missing input") {
        let outpath;

        if matches.is_present("inplace") {
            // use tempdir::TempDir;
            todo!("--inplace");
        } else if matches.is_present("subdir") {
            let subdir = matches.value_of("subdir").expect("missing subdir");
            outpath = match pdf_into_subdir(inpath, subdir) {
                Some(p) => p,
                None => {
                    eprintln!(
                        "Cannot process {:?} because the computed output is invalid",
                        inpath
                    );
                    continue;
                }
            };
            if !dry_run {
                let subpath = match pdf_subdir(inpath, subdir) {
                    Some(p) => p,
                    None => {
                        eprintln!(
                            "Cannot process {:?} because the computed subdir is invalid",
                            inpath
                        );
                        continue;
                    }
                };
                if let Err(e) = std::fs::create_dir_all(&subpath) {
                    eprintln!("Cannot create {:?}: {:?}", subpath, e);
                    continue;
                }
            }
        } else {
            outpath = match pdf_to_cmp_pdf(inpath) {
                Some(p) => p,
                None => {
                    eprintln!(
                        "Cannot process {:?} because the computed output is invalid",
                        inpath
                    );
                    continue;
                }
            };
        }

        if verbose {
            println!("Processing {:?} -> {:?}", inpath, outpath);
        }

        if dry_run {
            let mut cmd = dry_run_command(inpath, outpath);

            if verbose {
                println!("Running {:?}", cmd);
            }

            cmd.status().expect("failed to execute command");
        } else {
            let mut cmd = gs_command(inpath, outpath);

            if verbose {
                println!("Running {:?}", cmd);
            }

            cmd.status().expect("failed to execute command");
        }
    }
}
