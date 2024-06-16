use serde_json::to_string_pretty;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::{fs::File, io::Error, io::ErrorKind};
use crate::note::{Entry, Category};

pub enum DatabaseAction {
    Add,
    Remove(usize),
    Modify(usize)
}

#[derive(PartialEq)]
enum Mode {
    Ascending,
    Descending
}

enum SortBy{
    Unsorted,
    DateCreated(Mode),
    DateModified(Mode),
    Title(Mode)
}

const DATABASE_FILE: &str = "database.json";
const DATABASE_FILE_FILTERED: &str = "database_filtered.json";

pub fn refresh_json_database(entry: Option<Entry>, action: DatabaseAction) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .open(DATABASE_FILE)
        .unwrap();

    let mut file_content = String::new();

    file.read_to_string(&mut file_content).unwrap();

    let mut json_values: Vec<Entry> = vec![];

    if !file_content.is_empty() {
        json_values = serde_json::from_str(&file_content)
            .expect("The json file should be formatted correctly");

        println!("OK");
    }

    match action {
        DatabaseAction::Add => json_values.push(entry.unwrap()),

        DatabaseAction::Modify(key) => {
            if key >= json_values.len() {return Err(Error::from(ErrorKind::InvalidInput));}
            json_values[key].set_entry(entry.unwrap());
        }

        DatabaseAction::Remove(key) =>{
            if key >= json_values.len() {return Err(Error::from(ErrorKind::InvalidInput));}
            json_values.remove(key);
        } 
    }

    file.set_len(0).unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();
    
    let json = to_string_pretty(&json_values).unwrap();
    println!("{}, {:?}", json, json_values);
    file.write_all(&json.as_bytes())
        .expect("Error writing file!");

    Ok(())
}


pub fn generate_filtered_json(category: Category, sort: SortBy) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .open(DATABASE_FILE_FILTERED)
        .unwrap();

    let mut file_content = String::new();

    file.read_to_string(&mut file_content).unwrap();

    let mut json_values: Vec<Entry> = vec![];

    if !file_content.is_empty() {
        json_values = serde_json::from_str(&file_content)
            .expect("The json file should be formatted correctly");

        println!("OK");
    }

    if category != Category::All {
        json_values = json_values.into_iter().filter(|item|{
            item.get_category() == category
        }).collect();
    }
    
    match sort {
        SortBy::DateCreated(sorting_mode) => {
            json_values.sort_by(|entry_a, entry_b|{
                entry_a.date_created.cmp(&entry_b.date_created)
            });
            if sorting_mode == Mode::Descending {
                json_values.reverse();
            } 
        }

        SortBy::DateModified(sorting_mode) => {
            json_values.sort_by(|entry_a, entry_b|{
                entry_a.date_modified.cmp(&entry_b.date_created)
            });
            if sorting_mode == Mode::Descending {
                json_values.reverse();
            } 
        }

        SortBy::Title(sorting_mode) => {
            json_values.sort_by(|entry_a, entry_b|{
                entry_a.title().cmp(&entry_b.title())
            });
            if sorting_mode == Mode::Descending {
                json_values.reverse();
            } 
        }
        _ => ()
    }

    file.set_len(0).unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();
    
    let json = to_string_pretty(&json_values).unwrap();
    println!("{}, {:?}", json, json_values);
    file.write_all(&json.as_bytes())
        .expect("Error writing file!");

    Ok(())
}


pub fn current_entry_number() -> usize {
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .open(DATABASE_FILE);
    
    if let Ok(mut inside_file) = file {
        let mut json_values: Vec<Entry> = vec![];
        let mut file_content = String::new();
        inside_file.read_to_string(&mut file_content).unwrap();
        if !file_content.is_empty() {
            json_values = serde_json::from_str(&file_content)
                .expect("The json file should be formatted correctly");
            return json_values.len() + 1;
        }
    }
    return 1;
}

pub fn create_list() -> Result<(), Error> {
    File::create(DATABASE_FILE)?;
    Ok(())
}