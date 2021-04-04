//! clap_generate implementation for markdown.
extern crate clap;
extern crate clap_generate;

use clap::App;
use clap_generate::Generator;
use std::io::Write;

// A macro to automatically unwraps `write!`.
macro_rules! w {
    ($($arg:tt)*) => {
        write!($($arg)*).expect("Failed to write to generated file")
    }
}

/// `clap_generate::Generator` implementer for Markdown.
pub struct Markdown;

impl Generator for Markdown {
    fn file_name(name: &str) -> String {
        format!("{}.md", name)
    }

    fn generate(app: &App, buf: &mut dyn Write) {
        w!(buf, "{}", app);
    }
}

/// Write markdown to specified buffer. (Wraps `clap_generate::generate`.)
pub fn write_markdown<S: Into<String>>(app: &mut App, bin_name: S, buf: &mut dyn Write) {
    clap_generate::generate::<Markdown, _>(app, bin_name, buf);
}

/// Write markdown to `String`. (Wraps `write_markdown`.)
pub fn to_markdown<S: Into<String>>(
    app: &mut App,
    bin_name: S,
) -> String {
    let mut buf = Vec::new();
    write_markdown(app, bin_name, &mut buf);
    String::from_utf8_lossy(&buf).into_owned()
}

/// Write markdown to specified dir. (Wraps `clap_generate::generate_to`.)
pub fn write_markdown_to<S: Into<String>, T: Into<std::ffi::OsString>>(
    app: &mut App,
    bin_name: S,
    out_dir: T,
) {
    clap_generate::generate_to::<Markdown, _, _>(app, bin_name, out_dir);
}

/// Print markdown to stdout. (Wraps `write_markdown`.)
pub fn print_markdown<S: Into<String>>(app: &mut App, bin_name: S) {
    write_markdown(app, bin_name, &mut std::io::stdout());
}
