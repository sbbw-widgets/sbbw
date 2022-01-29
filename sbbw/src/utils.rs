use std::{path::PathBuf, fs};

pub fn get_config_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap();
    path.push("sbbw");
    fs::create_dir_all(&path).unwrap();
    path
}
pub fn get_widgets_path() -> PathBuf {
    let mut path = get_config_path();
    path.push("widgets");
    fs::create_dir_all(&path).unwrap();
    path
}
pub fn get_widgets() -> Vec<String> {
    let paths = fs::read_dir(get_widgets_path()).unwrap();
    paths.filter_map(|path| {
        let path = path.unwrap().path();
        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_str().unwrap();
            Some(dir_name.to_string())
        } else {
            None
        }
    }).collect()
}
