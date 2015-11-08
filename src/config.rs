//! File Journal which implements handlers for a journal that's on the file disk.
//! It's the default and only handler right now.
//!
//! Usage for bootstraping and saving:
//!
//!```
//!    use rjrn::config::Config;
//!    use rjrn::journal::Journal;
//!    use rjrn::file_journal::FileJournal;
//!
//!    let journal = FileJournal::new("name", "path");
//!    let config = Config { file_journals: vec![Box::new(journal)]};
//!    assert_eq!(config, Config::from_json(config.to_json()));
//!```

use journal::Journal;
use file_journal::FileJournal;

use rustc_serialize::json::{self, Json};
use std::collections::BTreeMap;
use std::error::Error;
use std::io::prelude::*;
use std::io;
use std::fs::{File, OpenOptions};
use std::path::{PathBuf};
use std::env;
use std::cmp::PartialEq;
const CONFIG_PATH: &'static str = ".rjrn.config";


// FIXME: memoize the result
fn config_path() -> PathBuf {
    match env::home_dir() {
        Some(ref p) => p.join(CONFIG_PATH),
        None => panic!("Impossible to get your home dir!")
    }
}

// FIXME: How to base that on the trait instead of specific type?
#[derive(Debug)]
pub struct Config {
    pub file_journals: Vec<Box<FileJournal>>
}

impl Config {
    pub fn to_json(&self) -> Json {
        let mut d: BTreeMap<String, Json> = BTreeMap::new();
        d.insert("journals".to_string(), Json::Array(self.file_journals.iter().map(|j| {
            j.to_json()
        }).collect()));
        Json::Object(d)
    }

    pub fn from_json(config: Json) -> Config {
        debug!("Read config: {:?}", config);
        let ref journals = config["journals"].as_array().unwrap();
        let file_journals = journals.iter()
            .filter(|j| j.find("type").unwrap().as_string().unwrap() == "FileJournal")
            .map(|j| Box::new(FileJournal::from_json(j))).collect();

       Config {
            file_journals: file_journals
        }
    }

    fn config_file() -> File {
        let get_file = || { OpenOptions::new().read(true).write(true).clone() };

        match get_file().open(config_path()) {
            Err(_) =>  {
                debug!("Creating new config file...");
                match get_file().create(true).truncate(true).open(config_path()) {
                    Ok(file) => file,
                    Err(why) =>
                        panic!("Couldn't create config file because: {}",
                               Error::description(&why))
                }
            },
            Ok(file) => file,
        }
    }

    pub fn load() -> Result<Config, String> {
        let mut file = Config::config_file();
        let mut s = String::new();

        match file.read_to_string(&mut s) {
            Err(why) => Err(format!("couldn't read {}",
                               Error::description(&why))),
            Ok(_) => {
                match s.len() {
                    0 => Ok(Config { file_journals: vec![] }),
                    _ => Ok(Config::from_json(Json::from_str(&s).unwrap()))

                }
            }
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let mut file  = Config::config_file();

        match writeln!(file, "{}", json::encode(&self.to_json()).unwrap()) {
            Err(why) => {
                Err(format!("Couldn't save the config file because: {}",
                       Error::description(&why)))
            },
            Ok(_) => {
                info!("Config saved.");
                Ok(())
            }
        }
    }

    pub fn add_journal(&mut self) -> Result<(), String> {
        println!("What type of journal you'd like to add: ");
        println!("(1) File Journal");
        println!("(??) TODO: Dropbox Journal");
        println!("(??) TODO: Trello Journal");

        let mut selected_option = String::new();

        loop {
            io::stdin().read_line(&mut selected_option)
                .ok()
                .expect("Failed to parse selection");

            debug!("Selected option: {}", selected_option);
            if selected_option.trim() == "1" {
                let mut journal = try!(FileJournal::bootstrap_cli());

                if self.file_journals.is_empty() {
                    journal.set_default();
                }

                self.file_journals.push(Box::new(journal));
                return Ok(())
            } else {
                println!("Failed to parse selection. Please try again:");
            }
        }
    }

    pub fn file_journal_with_name_or_default(&self, name: &str) ->
        Option<&Box<FileJournal>> {
            self.file_journals
                .iter()
                .fold(None, |result, journal| {
                    if journal.name() == name {
                        return Some(journal);
                    }

                    match result {
                        None => {
                            if journal.is_default() && name.len() == 0 { Some(journal) }
                            else { None }
                        },
                        Some(j) => Some(j)
                    }
                })
    }
}


impl PartialEq for Config {
     fn eq(&self, other: &Config) -> bool {
         for (a, b) in self.file_journals.iter().zip(other.file_journals.iter()) {
             if a != b {
                 return false;
             }
         }

         return true;
     }
 }
