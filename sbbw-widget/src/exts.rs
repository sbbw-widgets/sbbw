
use gtk::prelude::GtkWindowExt;
use wry::application::{window::Window, platform::unix::WindowExtUnix};

pub trait ManagedWindow {
    fn set_role(&self, name: &str, class: &str);
    // fn set_static_size(&self, width: i32, height: i32);
    fn stick(&self);
}

impl ManagedWindow for Window
{
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
        {
        }
    }
}
