use std::io;
use std::io::prelude::*;
use rjrn::entry::{Entry, EntryBuilder};
use rjrn::config::Config;
use rjrn::file_journal::FileJournal;
use rjrn::journal::Journal;
use cli_args::Args;

fn handle_version() -> Result<(), String> {
    println!("{}", env!("CARGO_PKG_VERSION"));
    Ok(())
}

fn handle_add_journal(config: &mut Config) -> Result<(), String> {
    try!(config.add_journal());
    match config.save() {
        Ok(_) => Ok(()),
        Err(_) => Err("Couldn't add journal".to_string())
    }
}

fn get_content_from_cli() -> String {
    println!("Please write your entry:");
    let stdin = io::stdin();
    let mut lines: Vec<String> = vec![];

    for line in stdin.lock().lines() {
        lines.push(line.unwrap());
    }

    lines.join("\n")
}

fn get_journal<'a>(config: &'a Config, name: &String) -> Result<&'a Box<FileJournal>, String> {
    match config.file_journal_with_name_or_default(name) {
        None => Err("Please add a journal".to_string()),
        Some(j) => Ok(j)
    }
}

fn add_new_entry(journal: &Box<FileJournal>, args: &Args) -> Result<(), String> {
    let content: String = match args.arg_content.len() {
        0 => get_content_from_cli(),
        _ =>  args.arg_content.join(" ")
    };

    let entry: Entry = try!(EntryBuilder::new()
                            .starred(args.flag_star)
                            .title(args.flag_title.clone())
                            .content(content)
                            .finalize());

    let id = entry.id().clone();
    match journal.upsert_entry(entry) {
        Ok(_) => {
            println!("entry add id: {:?}", id);
            Ok(())
        },
        Err(why) => Err(format!("Couldn't add entry because: {}", why))
    }
}

fn undo_last_entry(journal: &Box<FileJournal>) -> Result<(), String> {
    journal.undo_last_entry()
}

pub fn process_args(args: &Args) -> Result<(), String> {
    let mut config: Config = Config::load().unwrap();

    if args.flag_version { return handle_version(); }
    if args.flag_add { return handle_add_journal(&mut config); }

    let journal = try!(get_journal(&config, &args.flag_journal));
    if args.flag_undo { return undo_last_entry(&journal); }

    add_new_entry(&journal, args)
}
