//! Shrink PDF files using [Ghostscript](https://www.ghostscript.com/).
//!
//! This library provides a simple way to execute a Ghostscript command
//! which tries to optimize the resolution of embedded images in order
//! to reduce the file size.
//!
//! Ghostscript need to be already installed on your system.

use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(feature = "logging")]
use log::trace;

/// Replaces a `.pdf` extension with `.cmp.pdf`.
///
/// If there is no extension, or the extension is not `.pdf`, returns `None`.
///
/// # Examples
///
/// ```
/// # use pdfshrink::pdf_to_cmp_pdf;
/// let before = "some dir/subdir/name.pdf";
/// let after = "some dir/subdir/name.cmp.pdf";
/// assert_eq!(pdf_to_cmp_pdf(before), Some(after.into()));
/// ```
#[deprecated(
    since = "0.1.7",
    note = "Please use the `pdf_with_suffix` function instead"
)]
pub fn pdf_to_cmp_pdf<P>(inpath: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    let inpath = inpath.as_ref();
    let result = if inpath.extension() == Some("pdf".as_ref()) {
        Some(inpath.with_extension("cmp.pdf"))
    } else {
        None
    };
    #[cfg(feature = "logging")]
    trace!("pdf_to_cmp_pdf({:?}) = {:?}", inpath, result);
    result
}

/// Replaces a `.pdf` extension with `.<suffix>.pdf`.
///
/// If there is no extension, or the extension is not `.pdf`, returns `None`.
///
/// # Examples
///
/// ```
/// # use pdfshrink::pdf_with_suffix;
/// let before = "some dir/subdir/name.pdf";
/// let after = "some dir/subdir/name.shrunk.pdf";
/// assert_eq!(pdf_with_suffix(before, "shrunk"), Some(after.into()));
/// ```
pub fn pdf_with_suffix<P, Q>(inpath: P, suffix: Q) -> Option<PathBuf>
where
    P: AsRef<Path>,
    Q: AsRef<std::ffi::OsStr>,
{
    let inpath = inpath.as_ref();
    let suffix = suffix.as_ref();
    let mut new_extension = suffix.to_os_string();
    new_extension.push(".pdf");
    let result = if inpath.extension() == Some("pdf".as_ref()) {
        Some(inpath.with_extension(new_extension))
    } else {
        None
    };
    #[cfg(feature = "logging")]
    trace!("pdf_with_suffix({:?}, {:?}) = {:?}", inpath, suffix, result);
    result
}

/// Moves the file `inpath` into the subdirectory `subdir`.
///
/// If there is no extension, or the extension is not `.pdf`, returns `None`.
///
/// # Examples
///
/// ```
/// # use pdfshrink::pdf_into_subdir;
/// let before = "some dir/name.pdf";
/// let after = "some dir/subdir/name.pdf";
/// assert_eq!(pdf_into_subdir(before, "subdir"), Some(after.into()));
/// ```
pub fn pdf_into_subdir<P, Q>(inpath: P, subdir: Q) -> Option<PathBuf>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let inpath = inpath.as_ref();
    let subdir = subdir.as_ref();
    let result = if inpath.extension() == Some("pdf".as_ref()) {
        Some(
            inpath
                .parent()
                .unwrap_or("".as_ref())
                .join(subdir)
                .join(inpath.file_name()?),
        )
    } else {
        None
    };
    #[cfg(feature = "logging")]
    trace!("pdf_into_subdir({:?}, {:?}) = {:?}", inpath, subdir, result);
    result
}

/// Returns the subdirectory `subdir` sibling of `inpath`.
///
/// If there is no extension, or the extension is not `.pdf`, returns `None`.
///
/// # Examples
///
/// ```
/// # use pdfshrink::pdf_subdir;
/// let before = "some dir/name.pdf";
/// let after = "some dir/subdir";
/// assert_eq!(pdf_subdir(before, "subdir"), Some(after.into()));
/// ```
pub fn pdf_subdir<P, Q>(inpath: P, subdir: Q) -> Option<PathBuf>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let inpath = inpath.as_ref();
    let subdir = subdir.as_ref();

    let result = if inpath.extension()? == "pdf" {
        Some(inpath.parent().unwrap_or("".as_ref()).join(subdir))
    } else {
        None
    };
    #[cfg(feature = "logging")]
    trace!("pdf_subdir({:?}, {:?}) = {:?}", inpath, subdir, result);
    result
}

/// Ghostscript command to shrink `inpath` and write to `outpath`.
///
/// This command requires Ghostscript installed as a program `gs`.
pub fn gs_command<P, Q>(inpath: P, outpath: Q) -> Command
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    #[cfg(feature = "logging")]
    trace!("gs_command({:?}, {:?})", inpath.as_ref(), outpath.as_ref());
    let mut cmd = Command::new("gs");
    cmd.args(
        [
            "-q",
            "-dBATCH",
            "-dSAFER",
            "-dNOPAUSE",
            "-sDEVICE=pdfwrite",
            "-dCompatibilityLevel=1.4",
            "-dPDFSETTINGS=/ebook",
            "-dAutoRotatePages=/None",
            "-dColorImageDownsampleType=/Bicubic",
            "-dColorImageResolution=135",
            "-dGrayImageDownsampleType=/Bicubic",
            "-dGrayImageResolution=135",
            "-dMonoImageDownsampleType=/Bicubic",
            "-dMonoImageResolution=135",
        ]
        .iter(),
    )
    .arg(format!(
        "-sOutputFile={}",
        outpath.as_ref().to_string_lossy().to_string()
    ))
    .arg(inpath.as_ref().to_string_lossy().to_string());
    cmd
}

/// Command to simulate [`gs_command`].
///
/// Please see its documentation to know what it should do.
///
/// This command requires a program `args` which diagnoses the command line.
/// You can install for instance [args](https://github.com/FedericoStra/args)
/// or [argrs](https://github.com/FedericoStra/argrs) (in this case you must
/// symlink it to `args`).
pub fn dry_run_command<P, Q>(inpath: P, outpath: Q) -> Command
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    #[cfg(target_os = "windows")]
    trace!(
        "dry_run_command({:?}, {:?})",
        inpath.as_ref(),
        outpath.as_ref()
    );
    let mut cmd = Command::new("args");
    cmd.args(
        [
            "-q",
            "-dBATCH",
            "-dSAFER",
            "-dNOPAUSE",
            "-sDEVICE=pdfwrite",
            "-dCompatibilityLevel=1.4",
            "-dPDFSETTINGS=/ebook",
            "-dAutoRotatePages=/None",
            "-dColorImageDownsampleType=/Bicubic",
            "-dColorImageResolution=135",
            "-dGrayImageDownsampleType=/Bicubic",
            "-dGrayImageResolution=135",
            "-dMonoImageDownsampleType=/Bicubic",
            "-dMonoImageResolution=135",
        ]
        .iter(),
    )
    .arg(format!(
        "-sOutputFile={}",
        outpath.as_ref().to_string_lossy().to_string()
    ))
    .arg(inpath.as_ref().to_string_lossy().to_string());
    cmd
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdf_to_cmp_pdf() {
        #![allow(deprecated)]
        use pdf_to_cmp_pdf as f;
        for p in &["", "/", "./", "../"] {
            for d in &["", "dir/", "spaced dir/", "dotted.dir/"] {
                for n in &[
                    "name",
                    "spaced name",
                    "dotted.name",
                    ".hidden",
                    ".pdf", // this is not the extension
                    "strange'name",
                ] {
                    // valid case
                    let before = format!("{}{}{}.pdf", p, d, n);
                    let after = format!("{}{}{}.cmp.pdf", p, d, n);
                    assert_eq!(f(before), Some(after.into()));
                    // no extension
                    let before = format!("{}{}{}", p, d, n);
                    assert_eq!(f(before), None);
                    // wrong extension
                    let before = format!("{}{}{}.ext", p, d, n);
                    assert_eq!(f(before), None);
                }
            }
        }
    }

    #[test]
    fn test_pdf_with_suffix() {
        use pdf_with_suffix as f;
        for p in &["", "/", "./", "../"] {
            for d in &["", "dir/", "spaced dir/", "dotted.dir/"] {
                for s in &["cmp", "shrunk", "pdf.shrink", ".dotted"] {
                    for n in &[
                        "name",
                        "spaced name",
                        "dotted.name",
                        ".hidden",
                        ".pdf", // this is not the extension
                        "strange'name",
                    ] {
                        // valid case
                        let before = format!("{}{}{}.pdf", p, d, n);
                        let after = format!("{}{}{}.{}.pdf", p, d, n, s);
                        assert_eq!(f(before, s), Some(after.into()));
                        // no extension
                        let before = format!("{}{}{}", p, d, n);
                        assert_eq!(f(before, s), None);
                        // wrong extension
                        let before = format!("{}{}{}.ext", p, d, n);
                        assert_eq!(f(before, s), None);
                    }
                }
            }
        }
    }

    #[test]
    fn test_pdf_into_subdir() {
        use pdf_into_subdir as f;
        for p in &["", "/", "./", "../"] {
            for d in &["", "dir/", "spaced dir/", "dotted.dir/"] {
                for s in &["sub/", "spaced sub/", "dotted.sub/"] {
                    for n in &[
                        "name",
                        "spaced name",
                        "dotted.name",
                        ".hidden",
                        ".pdf", // this is not the extension
                        "strange'name",
                    ] {
                        // valid case
                        let before = format!("{}{}{}.pdf", p, d, n);
                        let after = format!("{}{}{}{}.pdf", p, d, s, n);
                        assert_eq!(f(before, s), Some(after.into()));
                        // no extension
                        let before = format!("{}{}{}", p, d, n);
                        assert_eq!(f(before, s), None);
                        // wrong extension
                        let before = format!("{}{}{}.ext", p, d, n);
                        assert_eq!(f(before, s), None);
                    }
                }
            }
        }
    }

    #[test]
    fn test_pdf_subdir() {
        use pdf_subdir as f;
        for p in &["", "/", "./", "../"] {
            for d in &["", "dir/", "spaced dir/", "dotted.dir/"] {
                for s in &["sub/", "spaced sub/", "dotted.sub/"] {
                    for n in &[
                        "name",
                        "spaced name",
                        "dotted.name",
                        ".hidden",
                        ".pdf", // this is not the extension
                        "strange'name",
                    ] {
                        // valid case
                        let before = format!("{}{}{}.pdf", p, d, n);
                        let after = format!("{}{}{}", p, d, s);
                        assert_eq!(f(before, s), Some(after.into()));
                        // no extension
                        let before = format!("{}{}{}", p, d, n);
                        assert_eq!(f(before, s), None);
                        // wrong extension
                        let before = format!("{}{}{}.ext", p, d, n);
                        assert_eq!(f(before, s), None);
                    }
                }
            }
        }
    }
}
