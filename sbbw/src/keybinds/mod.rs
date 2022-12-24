use crate::cmd::args::ShortcutsAction;
use colored::Colorize;
use sbbw_widget_conf::{generate_config_sbbw, KeyboardShortcuts, SbbwConfig};

use interactive::get_shortcut_interactive;
pub mod interactive;
pub mod validations;

pub fn process_arg(conf: &mut SbbwConfig, cmd: &ShortcutsAction) {
    match cmd {
        ShortcutsAction::List => {
            println!("{}", "Sbbw Daemon".green());
            println!("\n{}\n", "Configured shortcuts".yellow());
            for keybind in conf.shortcuts.iter() {
                println!(
                    "\t• <{}> => {:?} \"{}\"",
                    keybind.keys.join("-"),
                    keybind.action,
                    keybind.widget
                );
            }
        }
        ShortcutsAction::Interactive => {
            let ks = get_shortcut_interactive(conf);
            add_shortcut(conf, ks);
        }
        ShortcutsAction::Add {
            keys,
            action,
            widget,
            widget_args,
        } => {
            let ks = KeyboardShortcuts {
                keys: keys.to_owned(),
                widget: widget.to_owned(),
                action: action.clone(),
                widget_args: widget_args.to_owned().unwrap_or_default(),
                url: None,
            };
            add_shortcut(conf, ks);
        }
    }
}

fn add_shortcut(conf: &mut SbbwConfig, shortcut: KeyboardShortcuts) {
    conf.shortcuts.push(shortcut);
    if generate_config_sbbw(conf.clone()).is_err() {
        println!("\n\t{}", "Cannot update your configuration".red());
        std::process::exit(1);
    }
    println!("\n\t{}", "Succes add the shortcut".green());
}
