//! documentation in progress

mod database;
mod locale;
mod note;

use database::*;
use note::Entry;
use std::{fs::File, io::ErrorKind};
use wasm_bindgen::prelude::*;

const DATABASE_FILE: &str = "database.json";
const DATABASE_FILE_FILTERED: &str = "database_filtered.json";

#[wasm_bindgen]
pub fn add_note(entry: Entry) {
    {
        let file = File::open(DATABASE_FILE);
        match file {
            Err(error) => {
                match error.kind() {
                    // TODO: proper error handling
                    ErrorKind::NotFound => database::create_list().unwrap(),
                    ErrorKind::PermissionDenied => println!("Permission denied!"),
                    _ => {
                        println!("Unexpected error, Panicking!");
                        panic!();
                    }
                }
            }
            Ok(_) => (),
        }
    }
    refresh_json_database(Some(entry), database::DatabaseAction::Add)
        .expect("Couldn't invoke action on json Database");
}

#[wasm_bindgen]
pub fn modify_note(entry: Entry, key: usize) {
    {
        let file = File::open(DATABASE_FILE);
        match file {
            Err(error) => {
                match error.kind() {
                    // TODO: proper error handling
                    ErrorKind::NotFound => database::create_list().unwrap(),
                    ErrorKind::PermissionDenied => println!("Permission denied!"),
                    _ => {
                        println!("Unexpected error, Panicking!");
                        panic!();
                    }
                }
            }
            Ok(_) => (),
        }
    }
    refresh_json_database(Some(entry), database::DatabaseAction::Modify(key))
        .expect("Couldn't invoke action on json Database");
}

#[wasm_bindgen]
pub fn remove_note(key: usize) {
    {
        let file = File::open(DATABASE_FILE);
        match file {
            Err(error) => {
                match error.kind() {
                    // TODO: proper error handling
                    ErrorKind::NotFound => database::create_list().unwrap(),
                    ErrorKind::PermissionDenied => println!("Permission denied!"),
                    _ => {
                        println!("Unexpected error, Panicking!");
                        panic!();
                    }
                }
            }
            Ok(_) => (),
        }
    }
    refresh_json_database(None, database::DatabaseAction::Remove(key))
        .expect("Couldn't invoke action on json Database");
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn note_addtion_checking() {
        let note = Entry::new();
        add_note(note);
    }

    #[test]
    fn remove_entry_by_key() {
        let note = Entry::new();
        refresh_json_database(Some(note), DatabaseAction::Remove(0)).unwrap();
    }

    #[test]
    fn modify_entry() {
        let note = Entry::new();
        refresh_json_database(Some(note), DatabaseAction::Modify(0)).unwrap();
    }
}

// maybe add a macro to efficiently add categories
