mod gui;
mod service;
mod models;
mod repository;
mod schema;

fn main() {
    let mut gui = gui::GUI::new();
    gui.build();
    gui.show();
}