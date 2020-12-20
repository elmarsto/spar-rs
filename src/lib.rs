mod utils;
use wasm_bindgen::prelude::*;
use utils::*;
use js_sys::{ JsString };

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn translate() -> JsString {
    set_panic_hook();
    JsString::from("hello, world!")
}

