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
            // if config file exist on folder
            let mut config_path = path.clone();
            config_path.push("config.toml");
            if config_path.exists() {
                Some(path.file_name().unwrap().to_str().unwrap().to_string())
            } else {
                None
            }
        } else {
            None
        }
    }).collect()
}
