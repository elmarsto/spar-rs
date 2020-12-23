use wasm_bindgen::prelude::*;
use js_sys::{ Object, Array };


#[wasm_bindgen]
pub extern "C" {
    #[wasm_bindgen(extends = Object)]
    pub type Project;
    #[wasm_bindgen(extends = Object)]
    pub type SourceFile;
    #[wasm_bindgen(extends = Object)]
    pub type FunctionDeclaration;
    #[wasm_bindgen(extends = Object)]
    pub type VariableStatement;
    #[wasm_bindgen(extends = Object)]
    pub type VariableDeclaration;
    #[wasm_bindgen(extends = Object)]
    pub type EnumDeclaration;
    #[wasm_bindgen(extends = Object)]
    pub type EnumMember;
    #[wasm_bindgen(extends = Object)]
    pub type Parameter;

    #[wasm_bindgen(method, js_class="Project", js_name = getSourceFile)]
    pub fn Project_getSourceFile(this: &Project, name: &str) -> SourceFile;
    #[wasm_bindgen(method, js_class="SourceFile", js_name = getFunction)]
    pub fn SourceFile_getFunction(this: &SourceFile, name: &str) -> FunctionDeclaration;
    #[wasm_bindgen(method, js_class="SourceFile", js_name = getFunctions)]
    pub fn SourceFile_getFunctions(this: &SourceFile) -> Array;
    #[wasm_bindgen(method, js_class="SourceFile", js_name = getVariableStatements)]
    pub fn SourceFile_getVariableStatements(this: &SourceFile) -> Array;
    #[wasm_bindgen(method, js_class="SourceFile", js_name = getEnums)]
    pub fn SourceFile_getEnums(this: &SourceFile) -> Array;
    #[wasm_bindgen(method, js_class = "SourceFile", js_name = getInterfaces)]
    pub fn SourceFile_getInterfaces(this: &SourceFile) -> Array;
    #[wasm_bindgen(method, js_class = "SourceFile", js_name = getTypeAliases)]
    pub fn SourceFile_getTypeAliases(this: &SourceFile) -> Array;
    #[wasm_bindgen(method, js_class = "SourceFile", js_name = getClasses)]
    pub fn SourceFile_getClasses(this: &SourceFile) -> Array;



    #[wasm_bindgen(method, js_class="FunctionDeclaration", js_name = getBodyText)]
    pub fn FunctionDeclaration_getBodyText(this: &FunctionDeclaration) -> String;
    #[wasm_bindgen(method, js_class = "FunctionDeclaration", js_name = getParameters)]
    pub fn FunctionDeclaration_getParameters(this: &FunctionDeclaration) -> Array;
    #[wasm_bindgen(method, js_class = "FunctionDeclaration", js_name = getTypeParameters)]
    pub fn FunctionDeclaration_getTypeParameters(this: &FunctionDeclaration) -> Array;

}

