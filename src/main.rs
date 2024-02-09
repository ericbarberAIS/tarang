mod router;

mod components;
mod content;
mod generator;
mod pages;

use crate::components::app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::new().render();
}
