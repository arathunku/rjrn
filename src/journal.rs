use rustc_serialize::json::Json;
use entry::{Entry};
use uuid::Uuid;
use std::cmp::PartialEq;

pub trait Journal : PartialEq {
    fn bootstrap_cli() -> Result<Self, String>;
    fn from_json(journal: &Json) -> Self;
    fn to_json(&self) -> Json;
    fn set_default(&mut self);
    fn is_default(&self) -> bool;
    fn name(&self) -> &String;
    fn entries(&self) -> Result<Vec<Box<Entry>>, String>;
    fn upsert_entry(&self, entry: Entry) -> Result<(), String>;
    fn remove_entries(&self, ids: &Vec<Uuid>) -> Result<(), String>;
    fn undo_last_entry(&self) -> Result<(), String>;
}
