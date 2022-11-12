use std::env;
use std::path::PathBuf;
use std::fs;
use std::io::prelude::*;
use std::error::Error;

use regex::Regex;

pub fn base_directory_path() -> PathBuf {
    let path = match env::var("ADC_DATA_DIR") {
        Ok(path) => PathBuf::from(path),
        _ => default_data_directory(),
    };

    path.join("db")
}

pub fn default_data_directory() -> PathBuf {
    let home_dir = match env::var("HOME") {
        Ok(home) => home,
        _ => String::from(""),
    };

    PathBuf::from(home_dir)
        .join(".alamo")
}

pub fn ensure_base_directory_exists() -> Result<(), Box<dyn Error>> {
    let path = base_directory_path();
    fs::create_dir_all(&path)?;

    Ok(())
}

/// returns a list of all cinema files from the given path
pub fn list_cinema_files(path: PathBuf) -> Vec<PathBuf> {
    fs::read_dir(path)
        .unwrap()
        .filter(|entry| {
            match entry {
                Ok(entry) => !entry.path().is_dir() && is_calendar_file(entry.path()),
                _ => false,
            }
        })
        .map(|entry| {
            if let Ok(entry) = entry {
                entry.path()
            } else {
                panic!("This shouldn't happen")
            }
        })
        .collect()
}

pub fn list_cinema_ids(path: PathBuf) -> Vec<String> {
    lazy_static! {
         static ref RE: Regex = Regex::new(r"^(\d+)\.").unwrap();
    }

    list_cinema_files(path)
        .iter()
        .map(|path| {
            let filename = path.file_name().unwrap().to_str().unwrap();

            match RE.captures(filename) {
                Some(cap) => cap[1].to_string(),
                None => panic!("Filename not parsable. FAIL."),
            }
        })
        .collect()
}

/// private function that, given a path
/// returns whether it's a calendar file by inspecting the filename
fn is_calendar_file(path: PathBuf) -> bool {
    lazy_static! {
         static ref RE: Regex = Regex::new(r"\.calendar\.json$").unwrap();
    }

    RE.is_match(path.to_str().unwrap())
}

/// Given a cinema ID,
/// construct a path to a the json file in the db directory
pub fn calendar_path_for_cinema_id(cinema_id: &str) -> PathBuf {
    // the db directory is ~/.alamo-movies/db
    let mut filename = String::from(cinema_id);
    filename.push_str(".calendar.json");

    base_directory_path().join(filename)
}

/// given the ID of the cinema and string data (from the web API)
/// write it to the spot
pub fn write_calendar_file(cinema_id: &str, data: &str) -> Result<(), Box<dyn Error>> {
    ensure_base_directory_exists()?;
    let filepath = calendar_path_for_cinema_id(cinema_id);
    let mut file = fs::File::create(filepath)?;

    file.write_all(data.as_bytes())?;

    Ok(())
}
