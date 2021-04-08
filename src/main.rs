#![feature(command_access)]

#[macro_use]
extern crate clap;
use clap::{AppSettings, Arg, ArgGroup};

use pdfshrink::*;

use log::{debug, info, warn};

fn main() {
    let app = app_from_crate!()
        .setting(AppSettings::UnifiedHelpMessage)
        .help_message("Print help information") // Imperative form
        .version_message("Print version information") // Imperative form
        .after_help("The options --inplace, --rename and --subdir are mutually exclusive.")
        .arg(
            Arg::with_name("input")
                .multiple(true)
                .required(true)
                .value_name("INPUT")
                .help("Input PDF files to shrink"),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .multiple(true)
                .help("Increase the level of verbosity"),
        )
        .arg(
            Arg::with_name("inplace")
                .long("inplace")
                .short("i")
                .help("Replace the original file"),
        )
        .arg(
            Arg::with_name("rename")
                .long("rename")
                .short("r")
                .help("Save the output to a renamed file: *.pdf -> *.shrunk.pdf (defaut)"),
        )
        .arg(
            Arg::with_name("subdir")
                .long("subdir")
                .short("d")
                .value_name("SUBDIR")
                .help("Save the output in a subdirectory"),
        )
        .arg(
            Arg::with_name("debug")
                .long("debug")
                .hidden(true)
                .help("Debug the command line"),
        )
        .arg(
            Arg::with_name("dry-run")
                .long("dry-run")
                .short("n")
                .help("Do not actually run the commands, just show them"),
        )
        .group(ArgGroup::with_name("output").args(&["inplace", "rename", "subdir"]));

    let matches = app.get_matches();

    let debug = matches.is_present("debug");
    let dry_run = matches.is_present("dry-run");
    let verbose = matches.is_present("verbose");

    set_up_env_logger(verbose);

    // BEGIN DEBUG
    if debug {
        eprintln!("{:#?}", matches);
        eprintln!("---");
        for arg in &["input", "output", "inplace", "rename", "subdir", "verbose"] {
            debug!(
                "{} ({} {}): {:?}",
                arg,
                matches.occurrences_of(arg),
                matches.is_present(arg),
                matches.values_of(arg)
            );
        }
        if matches.is_present("inplace") {
            debug!("output mode: inplace");
        } else if matches.is_present("rename") {
            debug!("output mode: rename");
        } else if matches.is_present("subdir") {
            debug!(
                "output mode: subdir = {:?}",
                matches.value_of("subdir").unwrap_or("shrunk")
            );
        } else {
            debug!("no output mode specified, defaulting to --rename");
        }
        eprintln!();
    }
    // END DEBUG

    for inpath in matches.values_of("input").expect("missing input") {
        if verbose {
            debug!("Processing {:?}", inpath);
        }

        let outpath;

        if matches.is_present("inplace") {
            // use tempdir::TempDir;
            todo!("--inplace");
        } else if matches.is_present("subdir") {
            let subdir = matches.value_of("subdir").expect("missing subdir");
            outpath = match pdf_into_subdir(inpath, subdir) {
                Some(p) => p,
                None => {
                    warn!(
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
                        warn!(
                            "Cannot process {:?} because the computed subdir is invalid",
                            inpath
                        );
                        continue;
                    }
                };
                if let Err(e) = std::fs::create_dir_all(&subpath) {
                    warn!("Cannot create {:?}: {:?}", subpath, e);
                    continue;
                }
            }
        } else {
            outpath = match pdf_with_suffix(inpath, "shrunk") {
                Some(p) => p,
                None => {
                    warn!(
                        "Cannot process {:?} because the computed output is invalid",
                        inpath
                    );
                    continue;
                }
            };
        }

        info!("Compressing {:?} -> {:?}", inpath, outpath);

        let mut cmd = if dry_run {
            dry_run_command(inpath, outpath)
        } else {
            gs_command(inpath, outpath)
        };

        if verbose {
            // debug!("Running {:?}", cmd);
            let mut cmdline = String::from(cmd.get_program().to_string_lossy());
            for arg in cmd.get_args() {
                cmdline.push_str(&format!(" {}", shell_escape::escape(arg.to_string_lossy())));
            }
            debug!("{}", cmdline);
        }

        let output = cmd.output().expect("failed to execute command");
        if !output.stdout.is_empty() {
            info!(
                "STDOUT:\n{}",
                String::from_utf8_lossy(&output.stdout).trim_end()
            );
        }
        if !output.stderr.is_empty() {
            debug!(
                "STDERR:\n{}",
                String::from_utf8_lossy(&output.stderr).trim_end()
            );
        }
    }
}

/*
fn set_up_logging(verbose: bool) {
    use fern::colors::{Color, ColoredLevelConfig};

    // configure colors for the whole line
    let line_colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        // we actually don't need to specify the color for debug and info, they are white by default
        // .info(Color::White)
        .debug(Color::Magenta)
        // depending on the terminals color scheme, this is the same as the background color
        .trace(Color::BrightBlack);

    // configure colors for the name of the level.
    // since almost all of them are the same as the color for the whole line, we
    // just clone `line_colors` and overwrite our changes
    let level_colors = line_colors.clone().info(Color::Green);

    // here we set up our fern Dispatch
    fern::Dispatch::new()
        // .format(move |out, message, record| {
        //     out.finish(format_args!(
        //         "{fg_color}[{date}][{target}][{level}{fg_color}] {message}\x1B[0m",
        //         fg_color = format_args!(
        //             "\x1B[{}m",
        //             line_colors.get_color(&record.level()).to_fg_str()
        //         ),
        //         date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        //         target = record.target(),
        //         level = level_colors.color(record.level()),
        //         message = message,
        //     ));
        // })
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{fg_color}[{level:>5}{fg_color}] {message}\x1B[0m",
                fg_color = format_args!(
                    "\x1B[{}m",
                    line_colors.get_color(&record.level()).to_fg_str()
                ),
                level = level_colors.color(record.level()),
                message = message,
            ));
        })
        // set the default log level. to filter out verbose log messages from dependencies, set
        // this to Warn and overwrite the log level for your crate.
        .level(if verbose {
            log::LevelFilter::Trace
        } else {
            log::LevelFilter::Info
        })
        // change log levels for individual modules. Note: This looks for the record's target
        // field which defaults to the module path but can be overwritten with the `target`
        // parameter:
        // `info!(target="special_target", "This log message is about special_target");`
        // .level_for("pretty_colored", log::LevelFilter::Trace)
        // output to stdout
        .chain(std::io::stderr())
        .apply()
        .unwrap();
}
*/

fn set_up_env_logger(verbose: bool) {
    use std::io::Write;
    env_logger::Builder::new()
        .filter_level(if verbose {
            log::LevelFilter::Trace
        } else {
            log::LevelFilter::Info
        })
        .format_timestamp(None)
        .format_module_path(false)
        .format(|buf, record| {
            use env_logger::fmt::Color::*;
            use log::Level::*;

            let mut gray = buf.style();
            gray.set_color(Black).set_intense(true);

            let mut level_style = buf.style();
            match record.level() {
                Error => level_style.set_color(Red).set_intense(true),
                Warn => level_style.set_color(Yellow).set_intense(true),
                Info => level_style.set_color(Green).set_intense(true),
                Debug => level_style.set_color(Magenta),
                Trace => level_style.set_color(Black).set_intense(true),
            };

            let message_style = if record.level() == Info {
                buf.style()
            } else {
                level_style.clone()
            };

            writeln!(
                buf,
                "{}{level:>5}{} {message}",
                gray.value('['),
                gray.value(']'),
                level = level_style.value(record.level()),
                message = message_style.value(record.args())
            )
        })
        .init();
}
