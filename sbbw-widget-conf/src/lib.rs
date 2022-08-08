#[allow(unused_imports)]
use colored::*;
use serde::{
    de::{self, Deserializer},
    Deserialize, Serialize, Serializer,
};
use std::{fs, path::PathBuf};

fn deserialize_widget_size<'de, D>(de: D) -> Result<WidgetSize, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(de)?;
    match s.to_ascii_lowercase().as_str() {
        "max" => Ok(WidgetSize::Max),
        _ => {
            let v = s.parse::<f64>();
            if v.is_err() {
                return Err(de::Error::custom(format!(
                    "[{}] Invalid widget size (Cannot convert into f64): {}",
                    "Error".red().bold(),
                    s
                )));
            }
            Ok(WidgetSize::Value(v.unwrap()))
        } // _ => Err(serde::de::Error::custom("error trying to deserialize rotation policy config"))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum WidgetSize {
    Max,
    Value(f64),
}

impl Serialize for WidgetSize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            WidgetSize::Max => serializer.serialize_str("Max"),
            WidgetSize::Value(v) => serializer.serialize_str(&v.to_string()),
        }
    }
}

impl Default for WidgetSize {
    fn default() -> Self {
        WidgetSize::Max
    }
}

#[derive(Clone, Serialize, Default, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(default)]
pub struct AutoStartCommand {
    pub cmd: String,
    pub args: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(default)]
pub struct WidgetConfig {
    pub name: String,
    pub class_name: String, // TODO: add support
    #[serde(deserialize_with = "deserialize_widget_size")]
    pub width: WidgetSize,
    #[serde(deserialize_with = "deserialize_widget_size")]
    pub height: WidgetSize,
    pub x: f32,
    pub y: f32,
    pub transparent: bool, // Only works on Windows and Mac. For the linux users can be set with compositor
    pub blur: bool,
    pub always_on_top: bool,
    pub stick: bool,
    pub autostart: Vec<AutoStartCommand>,
}

impl Default for WidgetConfig {
    fn default() -> Self {
        WidgetConfig {
            name: "Internal".to_string(),
            class_name: "Internal_Class".to_string(),
            width: WidgetSize::Value(200.0),
            height: WidgetSize::Max,
            x: 0.0,
            y: 0.0,
            transparent: true,
            blur: true,
            always_on_top: true,
            stick: true,
            autostart: vec![],
        }
    }
}

impl WidgetConfig {
    pub fn new(name: String) -> Self {
        WidgetConfig {
            name: name.clone(),
            class_name: name.to_uppercase().replace(" ", "_"),
            ..Default::default()
        }
    }

    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    pub fn set_class_name(&mut self, class_name: String) -> &mut Self {
        self.class_name = class_name;
        self
    }

    pub fn set_size(&mut self, width: WidgetSize, height: WidgetSize) {
        self.width = width;
        self.height = height;
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn set_transparent(&mut self, transparent: bool) {
        self.transparent = transparent;
    }

    pub fn set_blur(&mut self, blur: bool) {
        self.blur = blur;
    }

    pub fn set_always_on_top(&mut self, allways_on_top: bool) {
        self.always_on_top = allways_on_top;
    }
}

fn validate_config_from_string(config: &str) -> Result<WidgetConfig, String> {
    match toml::from_str::<'_, WidgetConfig>(&config) {
        Ok(conf) => Ok(conf),
        Err(e) => Err(format!(
            "[{}] Config file is not valid: {}",
            "Error".red().bold(),
            e
        )),
    }
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
    validate_config_from_string(&conf_str.as_str())
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
pub fn get_widgets() -> Vec<String> {
    let paths = fs::read_dir(get_widgets_path()).unwrap();
    paths
        .filter_map(|path| {
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
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::WidgetSize;

    #[test]
    fn test_validate_config_toml() {
        let conf_path = PathBuf::from("/tmp/test_config.toml");
        let conf = super::validate_config_toml(conf_path).unwrap();
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
        let conf = super::validate_config_from_string(raw_conf).unwrap();

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
        let conf = super::validate_config_from_string(raw_conf).unwrap();

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
