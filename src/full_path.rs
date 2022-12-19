use std::path::PathBuf;

use crate::AppData;

pub fn get_full_path (app_data: &AppData, path: &str, static_: bool) -> PathBuf {
    let mut full_path = PathBuf::from(app_data.config.application.html.path.clone());
    if static_ {
        full_path.push(app_data.config.application.html.static_path.clone());
    }
    full_path.push(path);
    print!("Full path: {:?}", full_path);
    full_path
}