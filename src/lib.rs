//! Fast and easy queue abstraction.
//!
//! Provides an abstraction over a queue.  When the abstraction is used
//! there are these advantages:
//! - Fast
//! - [`Easy`]
//!
//! [`Easy`]: http://thatwaseasy.example.com


mod consts;
mod database;
mod locale;
mod note;

use database::*;
use note::{Category, Entry};
use std::{fs::File, io::ErrorKind};
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn add_note(entry: Entry) {
    {
        let file = File::open(consts::DATABASE_FILE);
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
        let file = File::open(consts::DATABASE_FILE);
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
        let file = File::open(consts::DATABASE_FILE);
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

#[wasm_bindgen]
pub fn filter_by_mode(category: Category, sort_by: SortBy, sorting_mode: Mode) {
    generate_filtered_json(category, sort_by, sorting_mode)
        .expect("Could not initialize the sorted file!");
}

// Use a better and shorter way to convert to a tuple (wasm bindgen binds my gens!!)
#[wasm_bindgen]
pub fn unix_to_utc_date(unix_time: u64) -> JsValue {
    let (date, _) = locale::convert_unix_to_custom_date(unix_time);
    JsValue::from_str(&date)
}

#[wasm_bindgen]
pub fn unix_to_utc_plus_one(unix_time: u64) -> JsValue {
    let (_, time) = locale::convert_unix_to_custom_date(unix_time);
    JsValue::from_str(&time)
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
