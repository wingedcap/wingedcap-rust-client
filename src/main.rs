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
    #[cfg(feature = "desktop")]
    {
        let launcher = dioxus::LaunchBuilder::new()
            .with_cfg(dioxus::desktop::Config::default().with_menu(None));
        launcher.launch(app::App);
    }

    #[cfg(any(feature = "mobile", feature = "web"))]
    {
        dioxus::launch(app::App);
    }
}
