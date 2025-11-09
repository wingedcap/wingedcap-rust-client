#![windows_subsystem = "windows"]

mod app;

mod types;

mod utils;

mod storage;

mod manager;

mod ui;

mod components;

mod views;

fn main() {
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    {
        let launcher = dioxus::LaunchBuilder::new().with_cfg(dioxus::desktop::Config::default().with_menu(None));
        launcher.launch(app::App);
    }

    #[cfg(target_os = "android")]
    {
        dioxus::launch(app::App);        
    }
}
