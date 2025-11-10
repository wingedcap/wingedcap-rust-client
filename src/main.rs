#![windows_subsystem = "windows"]

mod app;

mod types;

mod constants;

mod utils;

mod storage;

mod manager;

mod ui;

mod components;

mod views;

fn main() {
    #[cfg(feature = "desktop")]
    {
        use dioxus::desktop::{LogicalSize, WindowBuilder};

        let window = WindowBuilder::new()
            .with_title(crate::constants::APP_NAME.to_string())
            .with_inner_size(LogicalSize::new(800.0, 800.0))
            .with_min_inner_size(LogicalSize::new(400.0, 800.0));

        let launcher = dioxus::LaunchBuilder::new().with_cfg(
            dioxus::desktop::Config::default()
                .with_menu(None)
                .with_window(window),
        );

        launcher.launch(app::App);
    }

    #[cfg(any(feature = "mobile", feature = "web"))]
    {
        dioxus::launch(app::App);
    }
}
