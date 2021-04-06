use std::path::{Path, PathBuf};
use std::process::Command;

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
pub fn pdf_to_cmp_pdf<P>(inpath: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    let inpath = inpath.as_ref();
    if inpath.extension()? == "pdf" {
        Some(inpath.with_extension("cmp.pdf"))
    } else {
        None
    }
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
    // let subdir = subdir.as_ref();
    if inpath.extension()? == "pdf" {
        Some(
            inpath
                .parent()
                .unwrap_or("".as_ref())
                .join(subdir)
                .join(inpath.file_name()?),
        )
    } else {
        None
    }
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
    // let subdir = subdir.as_ref();
    if inpath.extension()? == "pdf" {
        Some(inpath.parent().unwrap_or("".as_ref()).join(subdir))
    } else {
        None
    }
}

pub fn gs_command<P, Q>(inpath: P, outpath: Q) -> Command
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let mut cmd = Command::new("gs");
    cmd.args(
        [
            "-q",
            "-dNOPAUSE",
            "-dBATCH",
            "-dSAFER",
            "-dPDFA=2",
            "-dPDFACompatibilityPolicy=1",
            "-dSimulateOverprint=true",
            "-sDEVICE=pdfwrite",
            "-dCompatibilityLevel=1.4",
            "-dPDFSETTINGS=/ebook",
            "-dEmbedAllFonts=true",
            "-dSubsetFonts=true",
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

pub fn dry_run_command<P, Q>(inpath: P, outpath: Q) -> Command
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let mut cmd = Command::new("args");
    cmd.args(
        [
            "gs",
            "-q",
            "-dNOPAUSE",
            "-dBATCH",
            "-dSAFER",
            "-dPDFA=2",
            "-dPDFACompatibilityPolicy=1",
            "-dSimulateOverprint=true",
            "-sDEVICE=pdfwrite",
            "-dCompatibilityLevel=1.4",
            "-dPDFSETTINGS=/ebook",
            "-dEmbedAllFonts=true",
            "-dSubsetFonts=true",
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
