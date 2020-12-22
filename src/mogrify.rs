use wasm_bindgen::prelude::*;
use super::tsmorph_sys::*;

#[wasm_bindgen]
pub fn trans(proj: &Project) -> String {
    let source_file = proj.Project_getSourceFile("spar.d.ts");
    let function_declaration = source_file.SourceFile_getFunction("hello");
    function_declaration.FunctionDeclaration_getBodyText()
}