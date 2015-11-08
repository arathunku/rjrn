//! File Journal which implements handlers for a journal that's on the file disk.
//! It's the default and only handler right now.
//!
//! Usage for bootstraping and saving:
//!
//!```
//!    use rjrn::journal::Journal;
//!    use rjrn::file_journal::FileJournal;
//!    let journal = FileJournal::new("name", "path");
//!    assert_eq!(journal, FileJournal::from_json(&FileJournal::to_json(&journal)));
//!```

use rustc_serialize::json::{self, Json};
use std::collections::BTreeMap;
use std::io::{self, Write, Read};
use std::error::Error;
use std::fs::{self, OpenOptions, File};
use std::env;
use uuid::Uuid;

use journal::{Journal};
use entry::Entry;

#[derive(Debug)]
pub struct FileJournal {
    name: String,
    path: String,
    default: bool,
}

impl Journal for FileJournal {
    fn from_json(j: &Json) -> FileJournal {
     FileJournal {
         name: String::from(j.find("name").unwrap().as_string().unwrap()),
         path: String::from(j.find("path").unwrap().as_string().unwrap()),
         default: j.find("default").unwrap().as_boolean().unwrap(),
      }
    }

    fn to_json(&self) -> Json {
        let mut d: BTreeMap<String, Json> = BTreeMap::new();
        d.insert("name".to_string(), Json::String(self.name.clone()));
        d.insert("path".to_string(), Json::String(self.path.clone()));
        d.insert("type".to_string(), Json::String("FileJournal".to_string()));
        d.insert("default".to_string(), Json::Boolean(self.default));
        Json::Object(d)
    }

    // Used by the CLI when the user want to create a new journal of that type
    fn bootstrap_cli() -> Result<FileJournal, String> {
        println!("Name of the journal (default):");

        let mut name = String::new();
        io::stdin().read_line(&mut name)
            .ok()
            .expect("Failed to parse name");

        if name.trim().is_empty() {
            name.push_str("default");
        }

        let default_path = format!("rjrn-{}.json", name.trim());
        println!("Path of the journal (~/{}):", default_path);

        let mut path = String::new();
        io::stdin().read_line(&mut path)
            .ok()
            .expect("Failed to parse path");

        if path.trim().is_empty() {
            match env::home_dir() {
                Some(ref p) => {
                    Ok(FileJournal::new(name.trim(),
                                     p.join(default_path)
                                         .to_str().unwrap()))
                },
                None => Err("Impossible to get your home dir!".to_string())
            }
        } else {
            Ok(FileJournal::new(name.trim(), path.trim()))
        }
    }

    fn set_default(&mut self) {
        self.default = true
    }

    fn is_default(&self) -> bool {
        self.default
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn entries(&self) -> Result<Vec<Box<Entry>>, String> {
        let mut s = String::new();
        let mut file = self.content_file(false);

        match file.read_to_string(&mut s) {
            Err(why) => Err(format!("Couldn't read entries {}", Error::description(&why))),
            Ok(_) => {
                if s.is_empty() {
                    Ok(vec![])
                } else {
                    Ok(json::decode(&s).unwrap())
                }
            }
        }
    }

    fn upsert_entry(&self, entry: Entry) -> Result<(), String> {
        debug!("Add entry with title:: {:?} to - {:?}", entry.title(), self.path);
        let mut entries = try!(self.entries());

        match entries.iter().position(|e| e.id() == entry.id()) {
            Some(i) => entries.insert(i, Box::new(entry)),
            None => entries.push(Box::new(entry))
        }

        self.save_entries(&entries)
    }

    fn remove_entries(&self, ids: &Vec<Uuid>) -> Result<(), String> {
        self.save_entries(
            &try!(self.entries())
                  .into_iter()
                  .filter(|e| !ids.contains(e.id()))
                  .collect())
    }

    fn undo_last_entry(&self) -> Result<(), String> {
        match try!(self.entries()).iter().last() {
            Some(e) => {
                debug!("Removing from {}, entry with id: {}", self.path, e.id());
                self.remove_entries(&vec![e.id().clone()])
            },
            None => { Ok(()) }
        }
    }
}


// FIXME: inspect the error and print only relevant msg.
const FILE_ERROR_MSG: &'static str =
    "Couldn't open/create journal file because: \n
           - you don't have suffiecient permissions\n
           - there's already a file by that name in that location\n
           - path is invalid";

impl FileJournal {
    pub fn new(name: &str, path: &str) -> FileJournal {
        FileJournal::validate_path(path);

        FileJournal {
            name: name.to_string().clone(),
            path: path.to_string().clone(),
            default: false,
        }
    }

    fn validate_path(path: &str) {
        debug!("Validating path: {}", path);
        OpenOptions::new().clone().create(true).open(path)
            .ok()
            .expect(FILE_ERROR_MSG);
       fs::remove_file(path).ok().expect(FILE_ERROR_MSG);
    }

    fn content_file(&self, truncate: bool) -> File {
        OpenOptions::new().read(true).write(true).create(true).truncate(truncate).clone()
            .open(&self.path)
            .ok().
            expect(FILE_ERROR_MSG)
    }

    fn save_entries(&self, entries: &Vec<Box<Entry>>) -> Result<(), String> {
        let mut file = self.content_file(true);

        match writeln!(file, "{}", json::encode(&entries).unwrap()) {
            Err(why) => Err(format!("Couldn't save the config file because: {}",
                                    Error::description(&why))),
            Ok(_) => Ok(())
        }
    }
}

impl PartialEq for FileJournal {
     fn eq(&self, other: &FileJournal) -> bool {
         self.path == other.path
     }
 }
