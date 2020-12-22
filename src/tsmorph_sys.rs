use wasm_bindgen::prelude::*;
use js_sys::{ Object };


#[wasm_bindgen]
pub extern "C" {
    #[wasm_bindgen(extends = Object)]
    pub type Project;
    #[wasm_bindgen(extends = Object)]
    pub type SourceFile;
    #[wasm_bindgen(extends = Object)]
    pub type FunctionDeclaration;

    #[wasm_bindgen(method, js_class="Project", js_name = getSourceFile)]
    pub fn Project_getSourceFile(this: &Project, name: &str) -> SourceFile;
    #[wasm_bindgen(method, js_class="SourceFile", js_name = getFunction)]
    pub fn SourceFile_getFunction(this: &SourceFile, name: &str) -> FunctionDeclaration;
    #[wasm_bindgen(method, js_class="FunctionDeclaration", js_name = getBodyText)]
    pub fn FunctionDeclaration_getBodyText(this: &FunctionDeclaration) -> String;
}

