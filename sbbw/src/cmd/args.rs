use std::{ops::Deref, sync::Mutex};

use clap::{AppSettings, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use lazy_static::lazy_static;
use sbbw_widget_conf::{get_widgets, RpcDataRequest};

use crate::{AUTHORS, DESCRIPTION};

#[derive(Parser, Debug)]
#[clap(author = AUTHORS, version, about = DESCRIPTION)]
#[clap(setting(AppSettings::ArgRequiredElseHelp))]
#[clap(unset_setting(AppSettings::SubcommandRequiredElseHelp))]
pub struct ArgOpt {
    #[clap(short, long, default_value = "8111")]
    pub port: u16,
    #[clap(subcommand)]
    pub widget_cmd: Option<WidgetCommands>,
    #[clap(short, long)]
    pub show_windows: bool,
    #[clap(long)]
    pub no_fork: bool,
    #[clap(flatten)]
    pub verbose: Verbosity<InfoLevel>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum WidgetCommands {
    #[clap(help = "This run widget server and all features")]
    Run,
    Install {
        #[clap(help = "Repository origin, ex: User/RepoName")]
        repo: String,
        #[clap(validator = validate_name_install)]
        new_name: Option<String>,
        #[clap(subcommand)]
        service: Option<RepositoryService>,
    },
    Open {
        #[clap(validator = validate_widgets)]
        widget_name: String,
        params: Option<String>,
    },
    Close {
        #[clap(validator = validate_widgets)]
        widget_name: String,
        params: Option<String>,
    },
    Toggle {
        #[clap(validator = validate_widgets)]
        widget_name: String,
        params: Option<String>,
    },
    Test {
        #[clap(validator = validate_widgets)]
        widget_name: String,
        url: String,
        params: Option<String>,
    },
    Check {
        #[clap(validator = validate_widgets)]
        widget_name: String,
    },
}

#[derive(Subcommand, Default, Debug, Clone)]
pub enum RepositoryService {
    #[default]
    Github,
    GitLab,
    BitBucket,
}

lazy_static! {
    pub static ref ARGS: Mutex<ArgOpt> = Mutex::new(ArgOpt::parse());
}

pub fn get_args() -> &'static impl Deref<Target = Mutex<ArgOpt>> {
    &ARGS
}

fn validate_widgets(src: &str) -> Result<(), String> {
    let widgets = get_widgets();
    if widgets.iter().any(|w| w.as_str().trim() == src) {
        Ok(())
    } else {
        Err(format!(
            "The value not in widgets installed {:?}",
            widgets.join(", ")
        ))
    }
}

fn validate_name_install(src: &str) -> Result<(), String> {
    match validate_widgets(src) {
        Ok(_) => Err("The widget alredy exists".to_string()),
        Err(_) => Ok(()),
    }
}

pub fn to_request(widget_cmd: &WidgetCommands, url: String) -> Result<RpcDataRequest, String> {
    match widget_cmd {
        WidgetCommands::Open {
            widget_name,
            params,
        } => Ok(RpcDataRequest {
            url: format!("{}/{}/ui", url, widget_name),
            widget_name: widget_name.to_string(),
            widget_params: params.clone(),
            action: sbbw_widget_conf::RpcAction::Open,
        }),
        WidgetCommands::Close {
            widget_name,
            params,
        } => Ok(RpcDataRequest {
            url: format!("{}/{}/ui", url, widget_name),
            widget_name: widget_name.to_string(),
            widget_params: params.clone(),
            action: sbbw_widget_conf::RpcAction::Close,
        }),
        WidgetCommands::Toggle {
            widget_name,
            params,
        } => Ok(RpcDataRequest {
            url: format!("{}/{}/ui", url, widget_name),
            widget_name: widget_name.to_string(),
            widget_params: params.clone(),
            action: sbbw_widget_conf::RpcAction::Toggle,
        }),
        WidgetCommands::Test {
            widget_name,
            params,
            url: test_url,
        } => Ok(RpcDataRequest {
            url: test_url.to_string(),
            widget_name: widget_name.to_string(),
            widget_params: params.clone(),
            action: sbbw_widget_conf::RpcAction::Test,
        }),
        _ => Err("No valid Widget Command".to_string()),
    }
}
