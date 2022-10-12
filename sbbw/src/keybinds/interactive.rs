use std::{
    io::{self, Read, Write},
    str::FromStr,
    time::Duration,
};

use colored::Colorize;
use device_query::{DeviceQuery, DeviceState, Keycode};
use sbbw_widget_conf::{get_widgets, KeyboardShortcuts, RpcAction, SbbwConfig, WidgetConfig};

use super::validations::{accept, is_rpc_action, is_widget, MyBool};

pub fn ask<T>(question: &str, validation: impl Fn(&str) -> bool) -> Result<T, T::Err>
where
    T: FromStr,
{
    let mut is_valid = false;
    let mut answer = String::new();
    let mut stdin = io::stdin();
    while !is_valid {
        print!("\t{}: ", question.bright_blue());
        let mut buf = [0u8; 1024];
        io::stdout().flush().unwrap();
        if let Ok(c) = stdin.read(&mut buf) {
            if c > 0 {
                answer = String::from_utf8(buf[..c - 1].to_vec()).unwrap();
                is_valid = validation(answer.trim());
            } else {
                println!("\t{}", "You need write something".magenta());
            }
        } else {
            println!("\tAn error ocurred");
        }
    }
    T::from_str(&answer)
}

pub fn get_keys(conf: &SbbwConfig) -> Vec<String> {
    let mut show_instruction = true;
    let device_state = DeviceState::new();
    let shortcuts: Vec<Vec<Keycode>> = conf
        .shortcuts
        .iter()
        .map(|s| {
            s.keys
                .iter()
                .map(|k| Keycode::from_str(k.as_str()).unwrap())
                .collect::<Vec<Keycode>>()
        })
        .collect();

    loop {
        if show_instruction {
            print!("\n\tPress the keys for the keyboard shortcut: ");
            io::stdout().flush().unwrap();
        }
        std::thread::sleep(Duration::from_millis(100));
        let keys = device_state.get_keys();
        if !keys.is_empty() && keys.len() > 2 {
            println!("{keys:?}");
            io::stdout().flush().unwrap();
            if shortcuts.contains(&keys) {
                println!("\t{}", "This shortcuts alredy exists".magenta());
                show_instruction = true;
                continue;
            }
            let ok = ask::<MyBool>("Are these keys OK? [Yes|No]", accept)
                .map_err(|e| println!("\n\t{}", e.magenta()))
                .unwrap_or(MyBool(false));
            if !ok.0 {
                show_instruction = true;
                continue;
            }
            break keys.iter().map(|k| k.to_string()).collect::<Vec<String>>();
        } else {
            show_instruction = false;
        }
    }
}

pub fn get_shortcut_interactive(conf: &SbbwConfig) -> KeyboardShortcuts {
    let (names, _): (Vec<String>, Vec<WidgetConfig>) = get_widgets().into_iter().unzip();
    println!("{}\n", "Starting to get shortcuts".cyan());
    let action =
        ask::<RpcAction>("Action to be taken [open|close|toggle|test]", is_rpc_action).unwrap();
    let keys = get_keys(conf);
    let name = ask::<String>("Which widget do we call?", |a| is_widget(a, &names)).unwrap();
    let url: Option<String> = ask::<String>(
        "You have your own url (perfect for testing, this is optional, you can leave it empty)",
        |_| true,
    )
    .ok();
    let args = ask::<String>(
        "Do you need to pass arguments to the widget? (Optional)",
        |_| true,
    )
    .unwrap();
    KeyboardShortcuts {
        widget: name,
        widget_args: args,
        keys,
        action,
        url,
    }
}
