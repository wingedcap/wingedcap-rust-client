mod app;

mod types;

mod utils;

mod storage;

mod manager;

mod ui;

mod components;

mod views;

fn main() {
    dioxus::launch(app::App);
}
