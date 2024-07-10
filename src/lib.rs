use dominator::html;
use wasm_bindgen::prelude::*;


#[wasm_bindgen(start)]
pub fn main_js() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    dominator::append_dom(&dominator::get_id("app"), html!("li"));
}
