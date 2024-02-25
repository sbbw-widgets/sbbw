#![allow(dead_code)]

use std::cell::RefCell;

use indicatif::{ProgressBar, ProgressStyle};

use git2::{
    build::{CheckoutBuilder, RepoBuilder},
    FetchOptions, Progress, RemoteCallbacks,
};
use sbbw_exec::autostarts;
use sbbw_widget_conf::{exits_widget, get_widgets_path};

use crate::cmd::args;

use super::args::WidgetCommands;

struct State {
    progress: Option<Progress<'static>>,
    progress_bar: Option<ProgressBar>,
    current: usize,
}

fn print(state: &mut State) {
    if let Some(pb) = &state.progress_bar {
        pb.inc(state.current as u64);
    }
}

pub fn install_widget(cmd: WidgetCommands) -> Result<(), String> {
    if let WidgetCommands::Install {
        repo,
        new_name,
        service,
    } = cmd
    {
        let mut repo_split = repo.split('/');
        if repo_split.clone().count() == 2 {
            let svc_url = match service {
                Some(svc) => match svc {
                    args::RepositoryService::Github => "https://github.com",
                    args::RepositoryService::GitLab => "https://gitlab.com",
                    args::RepositoryService::BitBucket => "https://bitbucket.com",
                },
                None => "https://github.com",
            };
            let widget_name = if let Some(name) = new_name {
                name
            } else {
                repo_split.nth(1).unwrap().to_string()
            };
            if !exits_widget(widget_name.clone()) {
                let state = RefCell::new(State {
                    progress_bar: None,
                    progress: None,
                    current: 0,
                });
                let mut cb = RemoteCallbacks::new();
                cb.transfer_progress(|stats| {
                    let mut state = state.borrow_mut();
                    state.progress = Some(stats.to_owned());
                    print(&mut state);
                    true
                });

                let mut co = CheckoutBuilder::new();
                co.progress(|_, cur, total| {
                    let mut state = state.borrow_mut();
                    let pb = ProgressBar::new(total as u64)
                        .with_prefix(format!("Installing {} ...", &widget_name));
                    pb.set_style(
                        ProgressStyle::with_template(
                            "{prefix:.bold} [{wide_bar:.cyan/blue}] [{elapsed_precise}] ",
                        )
                        .unwrap()
                        .progress_chars("#>-"),
                    );
                    state.progress_bar = Some(pb);
                    state.current = cur;
                    print(&mut state);
                });

                let mut fo = FetchOptions::new();
                fo.remote_callbacks(cb);
                println!();

                let path_widget = get_widgets_path().join(&widget_name);
                let clone = RepoBuilder::new()
                    .fetch_options(fo)
                    .with_checkout(co)
                    .clone(&format!("{}/{}", svc_url, repo), &path_widget);

                if let Err(e) = clone {
                    let state = state.borrow_mut();
                    if let Some(pb) = &state.progress_bar {
                        pb.finish_with_message("Installation Failed");
                    }
                    Err(e.to_string())
                } else {
                    let state = state.borrow_mut();
                    if let Some(pb) = &state.progress_bar {
                        pb.finish_with_message("Widget has been installed");
                    }
                    autostarts();
                    Ok(())
                }
            } else {
                Err("The widget already exists".to_string())
            }
        } else {
            Err("Invalid repo parameter format, check --help".to_string())
        }
    } else {
        Err("Invalid install parameter format, check --help".to_string())
    }
}
