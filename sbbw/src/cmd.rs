use std::{ops::Deref, sync::Mutex};

use lazy_static::lazy_static;
use sbbw_widget_conf::get_widgets;
use structopt::StructOpt;

use crate::{AUTHORS, DESCRIPTION, VERSION};

#[derive(Debug, StructOpt)]
#[structopt(name = "Sbbw Daemon", about=DESCRIPTION, version=VERSION, author=AUTHORS)]
pub struct ArgOpt {
    #[structopt(short, long, default_value = "8111")]
    pub port: u16,
    #[structopt(short, long, parse(try_from_str = validate_widgets))]
    pub open: Option<String>,
    #[structopt(short, long)]
    pub close: Option<String>,
    #[structopt(short, long)]
    pub toggle: Option<String>,
    #[structopt(long)]
    pub test: Option<Vec<String>>,
    #[structopt(long)]
    pub check_config: Option<String>,
    #[structopt(short, long)]
    pub show_windows: bool,
    #[structopt(long, parse(from_flag = std::ops::Not::not))]
    pub no_fork: bool,
}

impl Default for ArgOpt {
    fn default() -> Self {
        ArgOpt::from_args()
    }
}

lazy_static! {
    pub static ref ARGS: Mutex<ArgOpt> = Mutex::new(ArgOpt::default());
}

pub fn get_args() -> &'static impl Deref<Target = Mutex<ArgOpt>> {
    &ARGS
}

fn validate_widgets(src: &str) -> Result<String, structopt::clap::Error> {
    let widgets = get_widgets();
    if let Some(w) = widgets.iter().find(|w| w.as_str().trim() == src) {
        Ok(w.to_string())
    } else {
        Err(structopt::clap::Error::with_description(
            format!(
                "The value not in widgets installed {:?}",
                widgets
                    .iter()
                    .map(|w| format!("\t- {:?}", w))
                    .collect::<Vec<String>>()
                    .join("\n")
            )
            .as_str(),
            structopt::clap::ErrorKind::InvalidValue,
        ))
    }
}
