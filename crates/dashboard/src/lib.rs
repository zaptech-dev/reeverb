mod api;
mod app;
mod pages;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    leptos::mount::mount_to_body(app::App);
}
