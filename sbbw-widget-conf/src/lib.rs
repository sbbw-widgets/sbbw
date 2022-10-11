mod sbbw_conf;
mod widget_conf;

use colored::*;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

pub use sbbw_conf::prelude::*;
pub use widget_conf::*;

#[derive(Clone, Serialize, Default, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[serde(default)]
pub struct AutoStartCommand {
    pub cmd: String,
    pub args: Vec<String>,
}

fn validate_config_from_string(config: &str) -> Result<WidgetConfig, String> {
    match toml::from_str::<'_, WidgetConfig>(config) {
        Ok(conf) => Ok(conf),
        Err(e) => Err(format!(
            "[{}] Config file is not valid: {}",
            "Error".red().bold(),
            e
        )),
    }
}
pub fn exits_widget(widget_name: String) -> bool {
    let (widgets, _): (Vec<String>, Vec<WidgetConfig>) = get_widgets().iter().cloned().unzip();
    let exists = widgets.contains(&widget_name);
    if exists {
        let path_conf = get_widgets_path().join(widget_name).join("config.toml");
        return path_conf.exists();
    }
    false
}
pub fn validate_config_toml(conf_path: PathBuf) -> Result<WidgetConfig, String> {
    if !conf_path.exists() {
        return Err(format!(
            "[{}] Config file for window not found: {}",
            "Error".red().bold(),
            conf_path.display()
        ));
    }
    let conf_str = std::fs::read_to_string(conf_path).unwrap();
    validate_config_from_string(conf_str.as_str())
}

pub fn generate_config_sbbw(cfg: SbbwConfig) -> Result<(), std::io::Error> {
    let mut path = get_config_path();
    path.push("config.toml");

    fs::write(
        path.to_str().unwrap(),
        toml::to_string(&cfg).unwrap().as_str(),
    )
}

pub fn get_config_sbbw() -> Result<SbbwConfig, String> {
    let mut path = get_config_path();
    path.push("config.toml");

    let raw_str = fs::read_to_string(path.to_str().unwrap()).unwrap_or_default();
    match toml::from_str::<SbbwConfig>(raw_str.as_str()) {
        Ok(data) => Ok(data),
        Err(e) => Err(e.to_string()),
    }
}

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
pub fn get_widgets() -> Vec<(String, WidgetConfig)> {
    let paths = fs::read_dir(get_widgets_path()).unwrap();
    paths
        .filter_map(|path| {
            let path = path.unwrap().path();
            if path.is_dir() {
                // if config file exist on folder
                let mut config_path = path.clone();
                config_path.push("config.toml");
                if config_path.exists() {
                    let widget_name = path.file_name().unwrap().to_str().unwrap().to_string();
                    let widget_cfg = validate_config_toml(config_path).unwrap();
                    Some((widget_name, widget_cfg))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::WidgetSize;

    #[test]
    #[ignore = "Require Pre-configuration"]
    fn test_validate_config_toml() {
        let conf_path = PathBuf::from("/tmp/test_config.toml");
        let conf = validate_config_toml(conf_path).unwrap();
        assert_eq!(conf.name, "Test");
        assert_eq!(conf.class_name, "Test_Class");
        assert_eq!(conf.width, WidgetSize::Value(200.0));
        assert_eq!(conf.height, WidgetSize::Max);
        assert_eq!(conf.x, 0.0);
        assert_eq!(conf.y, 0.0);
        assert_eq!(conf.transparent, true);
        assert_eq!(conf.blur, true);
        assert_eq!(conf.always_on_top, true);
    }

    #[test]
    fn validate_config() {
        let raw_conf = r#"
            name = "Test"
            class_name = "Test_Class"
            width = "200.0"
            height = "300.0"
            x = 0.0
            y = 0.0
            transparent = true
            blur = true
            always_on_top=true
        "#;
        let conf = validate_config_from_string(raw_conf).unwrap();

        assert_eq!(conf.name, "Test");
        assert_eq!(conf.class_name, "Test_Class");
        assert_eq!(conf.width, WidgetSize::Value(200.0));
        assert_eq!(conf.height, WidgetSize::Value(300.0));
        assert_eq!(conf.x, 0.0);
        assert_eq!(conf.y, 0.0);
        assert_eq!(conf.transparent, true);
        assert_eq!(conf.blur, true);
        assert_eq!(conf.always_on_top, true);
    }

    #[test]
    fn validate_default_deserialization() {
        let raw_conf = r#"
            name = "Test"
            class_name = "Test Class"
            width = "200.0"
            x = 0.0
            always_on_top=true
        "#;
        let conf = validate_config_from_string(raw_conf).unwrap();

        assert_eq!(conf.name, "Test");
        assert_eq!(conf.class_name, "Test Class");
        assert_eq!(conf.width, WidgetSize::Value(200.0));
        assert_eq!(conf.height, WidgetSize::Max);
        assert_eq!(conf.x, 0.0);
        assert_eq!(conf.y, 0.0);
        assert_eq!(conf.transparent, true);
        assert_eq!(conf.blur, true);
        assert_eq!(conf.always_on_top, true);
    }
}
