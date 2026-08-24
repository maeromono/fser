use std::{collections::HashMap, path::PathBuf};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FserSave {
    pub last_state: HashMap<String, Bookmark>, // automatically created bookmarks
    pub bookmarks: HashMap<String, Bookmark>, // manually created bookmarks
}

#[derive(Serialize, Deserialize)]
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
