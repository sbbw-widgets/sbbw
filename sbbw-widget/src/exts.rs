#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd"
))]
use gtk::prelude::GtkWindowExt;

use sbbw_widget_conf::{WidgetConfig, WidgetSize};
use tao::{
    dpi::{LogicalSize, Size},
    monitor::MonitorHandle,
};
#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd"
))]
use wry::application::platform::unix::WindowExtUnix;

use tauri_plugin_vibrancy::Vibrancy;
use wry::application::window::Window;

pub trait ManagedWindow {
    fn set_role(&self, name: &str, class: &str);
    fn update_size(&self, conf: &WidgetConfig);
    fn blur_background(&self);
    // fn set_static_size(&self, width: i32, height: i32);
    fn stick(&self);
}

impl ManagedWindow for Window {
    fn set_role(&self, name: &str, class: &str) {
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        {
            let gtk_win = self.gtk_window();
            gtk_win.set_role(format!("{}_{}", name, class).as_str());
        }
    }

    fn update_size(&self, conf: &WidgetConfig) {
        let monitor_size = &self.current_monitor().unwrap().size();
        let width = match conf.width {
            WidgetSize::Max => monitor_size.width as f64,
            WidgetSize::Full => self
                .available_monitors()
                .fold(0.0, |acc, m: MonitorHandle| acc + m.size().width as f64),
            WidgetSize::Value(v) => v,
        };
        let height = match conf.height {
            WidgetSize::Max => monitor_size.height as f64,
            WidgetSize::Full => self
                .available_monitors()
                .fold(0.0, |acc, m: MonitorHandle| acc + m.size().height as f64),
            WidgetSize::Value(v) => v,
        };
        self.set_inner_size(Size::Logical(LogicalSize::new(width, height)))
    }

    fn blur_background(&self) {
        #[cfg(target_os = "windows")]
        self.apply_acrylic();
        #[cfg(target_os = "macos")]
        {
            use tauri_plugin_vibrancy::MacOSVibrancy;
            self.apply_vibrancy(tauri_plugin_vibrancy::MacOSVibrancy::AppearanceBased);
        }
    }

    fn stick(&self) {
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        {
            let gtk_win = self.gtk_window();
            gtk_win.stick();
        }
        #[cfg(target_os = "macos")]
        {}
    }
}
