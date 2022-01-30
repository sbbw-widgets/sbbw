use std::path::PathBuf;

use serde::{de::Deserializer, Deserialize, Serialize, Serializer};

fn deserialize_with<'de, D>(de: D) -> Result<WidgetSize, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(de)?;
    match s.to_ascii_lowercase().as_str() {
        "max" => Ok(WidgetSize::Max),
        _ => Ok(WidgetSize::Value(s.parse::<f32>().unwrap())),
        // _ => Err(serde::de::Error::custom("error trying to deserialize rotation policy config"))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum WidgetSize {
    Max,
    Value(f32),
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

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(default)]
pub struct WidgetConfig {
    pub name: String,
    pub class_name: String, // TODO: add support
    #[serde(deserialize_with = "deserialize_with")]
    pub width: WidgetSize,
    #[serde(deserialize_with = "deserialize_with")]
    pub height: WidgetSize,
    pub x: f32,
    pub y: f32,
    pub transparent: bool, // Only works on Windows and Mac. For the linux users can be set with compositor
    pub blur: bool,
    pub allways_on_top: bool,
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
            allways_on_top: true,
        }
    }
}

impl WidgetConfig {
    pub fn new(name: String) -> Self {
        WidgetConfig {
            name: name.clone(),
            class_name: name.to_uppercase(),
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

    pub fn set_allways_on_top(&mut self, allways_on_top: bool) {
        self.allways_on_top = allways_on_top;
    }
}

fn validate_config_from_string(config: &str) -> Result<WidgetConfig, String> {
    write_config_toml();
    match toml::from_str::<'_, WidgetConfig>(&config) {
        Ok(conf) => Ok(conf),
        Err(e) => Err(format!("Config file is not valid: {}", e)),
    }
}
pub fn validate_config_toml(conf_path: PathBuf) -> Result<WidgetConfig, String> {
    write_config_toml();
    if !conf_path.exists() {
        return Err(format!(
            "Config file for window not found: {}",
            conf_path.display()
        ));
    }
    let conf_str = std::fs::read_to_string(conf_path).unwrap();
    validate_config_from_string(&conf_str)
}
fn write_config_toml() {
    let conf_path = PathBuf::from("/tmp/test_config.toml");
    let mut conf = WidgetConfig::new("Test".to_string());
    conf.set_class_name("Test_Class".to_string());
    conf.set_size(WidgetSize::Value(200.0), WidgetSize::Max);
    conf.set_position(0.0, 0.0);
    conf.set_transparent(true);
    conf.set_blur(true);
    conf.set_allways_on_top(true);

    // save on /tmp/test_config.toml file
    let conf_str = toml::to_string(&conf).unwrap();
    std::fs::write(conf_path, conf_str).unwrap();
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
        assert_eq!(conf.allways_on_top, true);
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
            allways_on_top=true
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
        assert_eq!(conf.allways_on_top, true);
    }

    #[test]
    fn validate_default_deserialization() {
        let raw_conf = r#"
            name = "Test"
            class_name = "Test Class"
            width = "200.0"
            x = 0.0
            allways_on_top=true
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
        assert_eq!(conf.allways_on_top, true);
    }
}
