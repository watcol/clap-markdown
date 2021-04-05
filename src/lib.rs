//! clap_generate implementation for markdown.
extern crate clap;
extern crate clap_generate;

use clap::{App, Arg};
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

    // # TODO
    // ## App
    // -[x] name
    // -[x] bin_name
    // -[x] about
    // -[x] flags_with_no_heading
    // -[x] opts_with_no_heading
    // -[x] positionals
    // -[ ] arg_conflicts_with
    // -[ ] version (missong)
    // -[ ] author (missing)
    // -[ ] long_about (missing)
    // -[ ] long_flag
    // -[ ] short_flag
    // -[ ] visible_aliases
    // ## Arguments
    // -[x] name
    // -[x] about
    // -[x] short
    // -[x] long
    // -[ ] required (missing)
    // -[ ] multiple (missing)
    fn generate(app: &App, buf: &mut dyn Write) {
        let name = app.get_name();
        let about = app.get_about();
        let bin_name = app.get_bin_name();
        let flags = app.get_flags_with_no_heading().collect::<Vec<_>>();
        let have_flags = !flags.is_empty();
        let opts = app.get_opts_with_no_heading().collect::<Vec<_>>();
        let have_opts = !opts.is_empty();
        let args = app.get_positionals().collect::<Vec<_>>();
        let have_args = !args.is_empty();

        // Header
        w!(buf, "# {}\n", name);
        if let Some(about) = about {
            w!(buf, "{}\n", about);
        }
        w!(buf, "\n");

        // Table of Contents
        w!(buf, "## Table of Contents\n");
        w!(buf, "- [Usage](#usage)\n");
        if have_flags {
            w!(buf, "- [Flags](#flags)\n");
        }
        if have_opts {
            w!(buf, "- [Options](#options)\n");
        }
        if have_args {
            w!(buf, "- [Arguments](#arguments)\n");
        }
        w!(buf, "\n");

        // Usage
        w!(buf, "## Usage\n```");
        w!(buf, "{}", bin_name.unwrap_or(name));
        if have_flags {
            w!(buf, " [flags...]");
        }
        if have_opts {
            w!(buf, " [options...]");
        }
        for arg in args.iter() {
            w!(buf, " <{}>", arg.get_name());
        }
        w!(buf, "```\n\n");

        write_flags(buf, flags.as_slice());
        write_opts(buf, opts.as_slice());
        write_args(buf, args.as_slice());
    }
}

fn write_flags(buf: &mut dyn Write, flags: &[&Arg<'_>]) {
    if flags.is_empty() {
        return;
    }
    w!(buf, "## Flags\n");
    for flag in flags.iter() {
        let about = flag.get_about();
        let short = flag.get_short();
        let long = flag.get_long();
        if let Some(short) = short {
            w!(buf, "***-{}***, ", short);
        }
        if let Some(long) = long {
            w!(buf, "***--{}***", long);
        }
        w!(buf, "  \n");
        if let Some(about) = about {
            w!(buf, "<p style=\"text-indent:1em\">{}</p>\n\n", about);
        }
    }
}

fn write_opts(buf: &mut dyn Write, opts: &[&Arg<'_>]) {
    if opts.is_empty() {
        return;
    }
    w!(buf, "## Options\n");
    for opt in opts.iter() {
        let name = opt.get_name();
        let about = opt.get_about();
        let short = opt.get_short();
        let long = opt.get_long();
        if let Some(short) = short {
            w!(buf, "***-{}***, ", short);
        }
        if let Some(long) = long {
            w!(buf, "***--{}=\\<{}\\>***", long, name);
        }
        w!(buf, "  \n");
        if let Some(about) = about {
            w!(buf, "<p style=\"text-indent:1em\">{}</p>\n\n", about);
        }
    }
}

fn write_args(buf: &mut dyn Write, args: &[&Arg<'_>]) {
    if args.is_empty() {
        return;
    }
    w!(buf, "## Arguments\n");
    for arg in args.iter() {
        let name = arg.get_name();
        let about = arg.get_about();
        w!(buf, "***\\<{}\\>***  \n", name);
        if let Some(about) = about {
            w!(buf, "<p style=\"text-indent:1em\">{}</p>\n\n", about);
        }
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
