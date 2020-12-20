mod utils;

use wasm_bindgen::prelude::*;
use web_sys::*;
use utils::*;
use tastie::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn translate(ast: &Tastie) {
    set_panic_hook();
    ast.translate();
}

