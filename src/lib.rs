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
