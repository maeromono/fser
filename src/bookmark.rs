use std::{collections::HashMap, fs::{create_dir_all, read_to_string, write}, io::{Error, ErrorKind, Result}, path::PathBuf};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use toml::{from_str, to_string};

#[derive(Serialize, Deserialize, Default)]
pub struct FserSave {
    pub last_state: HashMap<String, Bookmark>, // automatically created bookmarks
    pub bookmarks: HashMap<String, Bookmark>, // manually created bookmarks
    // key = path to ebook btw
}

#[derive(Serialize, Deserialize, Default)]
pub struct Bookmark {
    pub page_index: usize,
    pub scroll: u16,
}

fn save_path() -> PathBuf {
    ProjectDirs::from("org.neocities", "enmkdev", "fser")
        .expect("cannot access directory")
        .data_dir()
        .join("bookmarks.toml")
}

pub fn save_bookmarks(save: &FserSave) -> Result<()> {
    let path = save_path();
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    let contents = to_string(save).map_err(|err| Error::new(ErrorKind::Other, err))?;
    write(path, contents)?;
    Ok(())
}

pub fn load_bookmarks() -> Result<FserSave> {
    let path = save_path();
    if !path.exists() {
        return Ok(FserSave::default())
    }
    let contents = read_to_string(path)?;
    let fsersave = from_str(&contents).map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
    Ok(fsersave)
}
