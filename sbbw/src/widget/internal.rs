use crate::get_widgets_path;
use colored::*;
use device_query::{DeviceQuery, DeviceState, Keycode};
use log::{info, trace, warn};
use sbbw_widget_conf::{get_config_sbbw, RpcAction, RpcDataRequest, SbbwConfig, WidgetConfig};
use std::{
    collections::HashMap,
    fs::OpenOptions,
    process::{Child, Command, Stdio},
    str::FromStr,
    thread,
    time::Duration,
};

use super::WIDGETS;

pub fn listen_keybinds(sbbw_cfg: SbbwConfig) {
    if !sbbw_cfg.shortcuts.is_empty() {
        thread::spawn(move || loop_listen_keybinds(sbbw_cfg));
    }
}

fn loop_listen_keybinds(cfg: SbbwConfig) {
    let device_state = DeviceState::new();
    let shortcuts = cfg
        .shortcuts
        .iter()
        .map(|s| {
            let keys = s
                .keys
                .iter()
                .map(|k| Keycode::from_str(k.as_str()).unwrap())
                .collect::<Vec<Keycode>>();
            (
                s.widget.clone(),
                s.action.clone(),
                s.url.clone(),
                s.widget_args.clone(),
                keys,
            )
        })
        .collect::<Vec<(String, RpcAction, Option<String>, String, Vec<Keycode>)>>();

    trace!("Starting listen keybinds");
    loop {
        let keys = device_state.get_keys();
        if keys.is_empty() {
            continue;
        }
        //
        // TODO: use rayon for get better performance
        //
        info!("Keys: {:?}", keys.clone());
        for (widget_name, action, url, args, shortcuts) in &shortcuts {
            if &keys == shortcuts {
                let widget_name = widget_name.clone();
                let action = action.clone();
                let args = args.clone();
                let url = url.clone();
                match action {
                    RpcAction::Test | RpcAction::Open => {
                        let mut widgets = WIDGETS.lock().unwrap();
                        open_widget(&mut widgets, &(widget_name, None), action, args, url)
                            .unwrap_or_default();
                    }
                    RpcAction::Close => {
                        let mut widgets = WIDGETS.lock().unwrap();
                        let widget_name = widget_name.clone();
                        close_widget(&mut widgets, &(widget_name, None)).unwrap_or_default();
                    }
                    RpcAction::Toggle => {
                        let mut widgets = WIDGETS.lock().unwrap();
                        toggle_widget(&mut widgets, &(widget_name, None), action, args, url)
                            .unwrap_or_default();
                    }
                }
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
}

pub fn open_widget(
    widgets: &mut HashMap<String, Child>,
    widget_data: &(String, Option<WidgetConfig>),
    action: RpcAction,
    widget_params: String,
    url: Option<String>,
) -> Result<(), String> {
    let (name, _) = widget_data;
    if widgets.contains_key(name) {
        warn!("[{}] Widget alredy opened", "Daemon".green().bold());
        return Err(format!("Widget {} already opened", name));
    }
    trace!("[{}] Open: {:?}", "Daemon".green().bold(), name);
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(get_widgets_path().join(name).join(".log"))
        .unwrap();

    let out = Stdio::from(file);
    let data_request = RpcDataRequest {
        action,
        widget_params: Some(widget_params),
        widget_name: name.clone(),
        url: if let Some(url) = url {
            url
        } else if let Ok(cfg) = get_config_sbbw() {
            format!("http://localhost:{}/{}/ui", cfg.port, name)
        } else {
            format!("http://localhost:8111/{}/ui", name)
        },
    };
    match Command::new("sbbw-widget")
        .args(data_request.get_args())
        .stderr(out)
        .spawn()
    {
        Ok(subprocess) => {
            trace!(
                "[{}] Widget \"{:?}\" added to opens",
                "Daemon".green().bold(),
                name
            );
            widgets.insert(name.to_string(), subprocess);
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}

pub fn close_widget(
    widgets: &mut HashMap<String, Child>,
    widget_data: &(String, Option<WidgetConfig>),
) -> Result<(), String> {
    if !widgets.contains_key(&widget_data.0) {
        log::error!("[{}] Widget not before open", "Daemon".green().bold());
        return Err("Widget not before open".to_string());
    }
    trace!("[{}] Close: {:?}", "Daemon".green().bold(), widget_data.0);
    if let Some(mut subprocess) = widgets.remove(&widget_data.0) {
        subprocess.kill().unwrap();
        drop(subprocess);
        trace!(
            "[{}] Widget process \"{:?}\" droped",
            "Daemon".green().bold(),
            widget_data.0
        );
    }
    Ok(())
}

pub fn toggle_widget(
    widgets: &mut HashMap<String, Child>,
    widget_data: &(String, Option<WidgetConfig>),
    action: RpcAction,
    widget_params: String,
    url: Option<String>,
) -> Result<(), String> {
    trace!(
        "[{}] Toggle widget \"{:?}\"",
        "Daemon".green().bold(),
        widget_data.0
    );
    if !widgets.contains_key(&widget_data.0) {
        trace!("[{}] Toggle widget (Open) ", "Daemon".green().bold());
        open_widget(widgets, widget_data, action, widget_params, url)
    } else {
        trace!("[{}] Toggle widget (Close)", "Daemon".green().bold());
        close_widget(widgets, widget_data)
    }
}
