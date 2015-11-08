use std::error;
use std::fmt;
use uuid::Uuid;
use chrono::*;
use std::str::FromStr;
use rustc_serialize::{Encodable, Encoder, Decodable, Decoder};


#[derive(Debug)]
pub enum EntryError { UnsuccessfulAdd }

impl fmt::Display for EntryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EntryError::UnsuccessfulAdd => write!(f, "Couldn't add entry - unsuccessful."),
        }
    }
}

impl error::Error for EntryError {
    fn description(&self) -> &str {
        match *self {
            EntryError::UnsuccessfulAdd => "Couldn't add entry - unsuccessful.",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            EntryError::UnsuccessfulAdd => None
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct DateTimeLocal(DateTime<UTC>);

impl Encodable for DateTimeLocal {
    fn encode<S: Encoder>(&self, e: &mut S) -> Result<(), S::Error> {
        e.emit_str(&self.0.to_rfc3339())
    }
}

impl Decodable for DateTimeLocal {
    fn decode<D: Decoder>(d: &mut D) -> Result<DateTimeLocal, D::Error> {
        Ok(DateTimeLocal(DateTime::<UTC>::from_str(&d.read_str()
                                                      .ok()
                                                      .expect("Couldn't parse DateTime from json"))
                         .ok()
                         .expect("Couldn't parse DateTime from str")))
    }
}

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug, Clone)]
pub struct Entry {
    id: Uuid,
    title: Option<String>,
    content: String,
    updated_at: Box<DateTimeLocal>,
    created_at: Box<DateTimeLocal>,
    starred: bool,
    tags: Vec<String>,
}

impl Entry {
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn title(&self) -> &Option<String> {
        &self.title
    }
}

#[derive(Clone)]
pub struct EntryBuilder {
    id: Uuid,
    title: Option<String>,
    content: String,
    updated_at: Box<DateTimeLocal>,
    created_at: Box<DateTimeLocal>,
    starred: bool,
    tags: Vec<String>,
}

impl EntryBuilder {
    pub fn new() -> EntryBuilder {
        let created_at = DateTimeLocal(UTC::now());

        EntryBuilder {
            id: Uuid::new_v4(),
            title: None,
            content: "".to_string(),
            updated_at: Box::new(created_at),
            created_at: Box::new(created_at),
            starred: false,
            tags: vec![],
        }
    }

    pub fn id(&mut self, id: Uuid) -> &mut EntryBuilder {
        self.id = id;
        self
    }

    pub fn title(&mut self, title: String) -> &mut EntryBuilder {
        self.title = Some(title);
        self
    }

    pub fn content(&mut self, content: String) -> &mut EntryBuilder {
        if content.starts_with("*") {
            self.content = content.chars().skip(1).collect();
            {
                self.starred(true);
            }
        } else {
            self.content = content;
        }

        if self.title == Some("".to_string()) || self.title == None {
            {
              self.set_title_from_content();
            }
        }
        self
    }

    pub fn starred(&mut self, state: bool) -> &mut EntryBuilder {
        self.starred = state;
        self
    }

    pub fn finalize(&self) -> Result<Entry, String> {
        // FIXME: do I really have to clone Strings here?
        if self.content != "" {
            Ok(Entry {
                id: self.id,
                title: self.title.clone(),
                content: self.content.clone(),
                updated_at: self.updated_at.clone(),
                created_at: self.created_at.clone(),
                starred: self.starred,
                tags: self.tags.clone(),
            })
        } else {
            Err("Content is empty!".to_string())
        }

    }

    fn set_title_from_content(&mut self) -> &mut EntryBuilder {
        let dividers = vec!['\n', '?', '!', '.'];
        self.title = Some(self.content
            .split(|c| dividers.contains(&c))
            .map(|c| c.to_string())
            .nth(0).unwrap_or(self.content.clone()));

        self
    }
}
