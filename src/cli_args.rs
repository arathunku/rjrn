extern crate docopt;

use self::docopt::Docopt;

const USAGE: &'static str ="
Journal.

The config will be kept at: `~/.rjrn.config`.
If the first word matches some journal name, the entry will be created in that journal, otherwise it goes to the default journal.

Usage:
  rjrn [--verbose]
  rjrn <content>... [--verbose]
  rjrn <content>... [--verbose]
  rjrn <content>... [--title=<title>] [--star] [--verbose] [--journal=<journal>]
  rjrn (--help | -h)
  rjrn (--version | -v)
  rjrn --undo [--verbose]
  rjrn --add [--verbose]

Options:
  --help -h                  Show this screen.
  --version -v               Show version.
  --title TITLE              The title of the new entry
  --journal JOURNAL          Name of the journal, if empty it selects default journal
  --star                     Marks the entry as favourite
  --add                      If you'd like to add a new journal file
  --undo                     Deletes last entry
  --verbose                  Print debug statements
";
#[derive(Debug, RustcDecodable)]
pub struct Args {
    pub flag_version: bool,
    pub flag_help: bool,
    pub flag_star: bool,
    pub flag_add: bool,
    pub flag_title: String,
    pub flag_journal: String,
    pub flag_undo: bool,
    pub flag_verbose: bool,
    pub arg_content: Vec<String>,
}

pub fn get() -> Args {
    Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit())
}
