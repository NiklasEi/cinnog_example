// This import is required for `wasm_bindgen` to do it's thing with our app
#[allow(unused_imports)]
#[allow(clippy::single_component_path_imports)]
use app;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}
