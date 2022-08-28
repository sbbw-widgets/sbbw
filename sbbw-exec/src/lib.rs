use std::{
    collections::HashMap,
    error::Error,
    fs::{read_dir, read_to_string, File},
    path::PathBuf,
};

use colored::Colorize;
use sbbw_widget_conf::{get_widgets_path, validate_config_toml};
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Params {
    pub method_id: isize,
    pub method: String,
    pub data: String,
}

fn generate_hash_from_file(path: PathBuf) -> Result<String, Box<dyn Error>> {
    let contents = read_to_string(path.as_path()).unwrap_or_else(|_| "".to_string());
    let content_to_hash = format!(
        "{}\n{}",
        path.file_name().unwrap().to_str().unwrap(),
        contents
    );
    let mut hash = Sha1::new();

    hash.update(content_to_hash.as_bytes());

    Ok(format!("{:x}", hash.finalize()))
}

pub fn exec_command(pwd: String, params: Vec<String>) -> Result<String, String> {
    let file = params.first().expect("The arguments cannot by empty");
    println!("{}", file);
    let mut args = params[1..].to_vec();
    if file.starts_with("./") {
        args.insert(0, file.to_string());
    }
    println!("{:?}", args);
    let output = if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(&["/C", "start"])
            .args(args)
            .output()
    } else if file.starts_with("./") {
        println!("Execute sh command");
        std::process::Command::new("sh")
            .arg("-c")
            .arg(&args.join(" "))
            .current_dir(pwd)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .output()
    } else {
        println!("Execute command");
        std::process::Command::new(file)
            .args(args)
            .current_dir(pwd)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .output()
    };

    let stdout = String::from_utf8_lossy(&output.as_ref().unwrap().stdout);
    let stderr = String::from_utf8_lossy(&output.as_ref().unwrap().stderr);

    if !stderr.is_empty() {
        println!("{}", stderr.red());
    }
    if !&stdout.is_empty() {
        println!("{}", stdout.green());
    }

    println!(
        "{}",
        String::from_utf8_lossy(&output.as_ref().unwrap().stdout)
    );

    Ok(stdout.to_string())
}

pub fn autostarts() {
    let config_dir = get_widgets_path();

    // Iterate over all widget files in the config directory
    for entry in read_dir(config_dir).unwrap() {
        let widget_path = entry.unwrap().path();
        let content_widget_lock = if widget_path.join("config.lock").exists() {
            let content = std::fs::read_to_string(&widget_path.join("config.lock"));
            content
                .unwrap()
                .split('\n')
                .filter(|x| !x.is_empty())
                .map(|l| {
                    let mut l = l.split(':');
                    let key = l.next().unwrap();
                    let value = l.next().unwrap();
                    (key.to_string(), value.to_string())
                })
                .collect::<HashMap<String, String>>()
        } else {
            File::create(widget_path.join("config.lock")).unwrap();
            HashMap::<String, String>::new() // filename: hashsum content
        }; // ignore this file on gitignore

        let mut new_content_widget_lock = HashMap::<String, String>::new();

        // generate hash for all files in autostart directory
        if widget_path.join("autostart").exists() {
            for autostart_file in read_dir(widget_path.join("autostart")).unwrap() {
                let autostart_file_path = autostart_file.unwrap().path();
                new_content_widget_lock.insert(
                    autostart_file_path
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                    generate_hash_from_file(autostart_file_path).unwrap(),
                );
            }
        }
        new_content_widget_lock.insert(
            "config.toml".to_string(),
            generate_hash_from_file(widget_path.join("config.toml")).unwrap(),
        );

        // compare old and new hash
        let mut changed = false;
        for (key, value) in new_content_widget_lock.iter() {
            changed =
                !(content_widget_lock.contains_key(key) && content_widget_lock[key] == *value);
            if changed {
                break;
            }
        }

        if changed {
            // Check if the file is a valid config file
            println!("{}", "Autostarting widget".bright_yellow());
            if widget_path.join("config.toml").exists() {
                let config_toml =
                    validate_config_toml(widget_path.join("config.toml")).unwrap_or_default();

                if !config_toml.autostart.is_empty() {
                    for autostart in config_toml.autostart {
                        let mut args = Vec::new();
                        args.push(autostart.cmd.to_string());
                        args.extend(autostart.args);

                        if !autostart.cmd.contains(".lua") {
                            match exec_command(
                                widget_path.join("autostart").to_str().unwrap().to_string(),
                                args,
                            ) {
                                Ok(_) => {}
                                Err(e) => {
                                    println!("{}", e);
                                }
                            }
                        }
                    }
                }
            }

            // write new lock file
            let mut new_lock = String::new();
            for (key, value) in new_content_widget_lock {
                new_lock.push_str(&format!("{}:{}\n", key, value));
            }
            std::fs::write(&widget_path.join("config.lock"), new_lock).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
