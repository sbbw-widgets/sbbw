use colored::Colorize;
use serde::{de::{self, Deserializer}, Serializer, Serialize, Deserialize};

use crate::AutoStartCommand;

fn deserialize_widget_size<'de, D>(de: D) -> Result<WidgetSize, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(de)?;
    match s.to_ascii_lowercase().as_str() {
        "max" => Ok(WidgetSize::Max),
        "full" => Ok(WidgetSize::Full),
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
    Full,
    Value(f64),
}

impl Serialize for WidgetSize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            WidgetSize::Max => serializer.serialize_str("Max"),
            WidgetSize::Full => serializer.serialize_str("Full"),
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
            class_name: name.to_uppercase().replace(' ', "_"),
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
