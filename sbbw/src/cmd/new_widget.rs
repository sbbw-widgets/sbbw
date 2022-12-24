use git2::Repository;
use sbbw_widget_conf::{exits_widget, get_widgets_path, write_widget_conf, WidgetConfig};

use crate::keybinds::{
    interactive::ask,
    validations::{accept, is_float, MyBool},
};

pub fn create_widget(name: &Option<String>) {
    let name: String = if let Some(n) = name {
        n.clone()
    } else {
        ask("Widget Name", true, |s| {
            !s.is_empty() && !exits_widget(s.to_string())
        })
        .unwrap()
    };

    let class_name = ask::<String>(
        "What class will the widget have (this is an identifier for the widget window)?",
        false,
        |s| !s.is_empty(),
    )
    .unwrap_or(name.clone().to_uppercase().replace(' ', "_"));
    let width = ask("What width will the widget be?", true, is_widget_size).unwrap();
    let height = ask("What height will the widget be?", true, is_widget_size).unwrap();
    let x = ask("What position in X will the widget have?", true, is_float).unwrap();
    let y = ask("What position in Y will the widget have?", true, is_float).unwrap();
    let transparent: MyBool =
        ask("Will the widget be transparent?", false, accept).unwrap_or(MyBool(false));
    let blur: MyBool = ask("Will the widget be blurry?", false, accept).unwrap_or(MyBool(false));
    let always_on_top: MyBool =
        ask("Will the widget always be on top?", false, accept).unwrap_or(MyBool(false));
    let stick: MyBool =
        ask("Will the widget be stuck on all desktops?", false, accept).unwrap_or(MyBool(false));

    let mut new_widget_path = get_widgets_path();
    new_widget_path.push(&name);
    std::fs::create_dir_all(new_widget_path.clone()).unwrap();
    match Repository::init(new_widget_path.clone()) {
        Ok(_) => {},
        Err(e) => {
            println!("Failed to create repository: {e}");
            return;
        },
    }

    let config = WidgetConfig {
        name: name.clone(),
        class_name,
        width,
        height,
        x,
        y,
        transparent: transparent.0,
        blur: blur.0,
        always_on_top: always_on_top.0,
        stick: stick.0,
        autostart: Vec::new(),
    };

    new_widget_path.push("config.toml");
    write_widget_conf(&mut new_widget_path, config);
    println!("The widget '{name}' has been successfully created in the path {new_widget_path:?}");
}

fn is_widget_size(s: &str) -> bool {
    !s.is_empty()
        || s.trim().to_lowercase() == "max"
        || s.trim().to_lowercase() == "full"
        || s.trim().parse::<f64>().map_or(false, |_| true)
}
