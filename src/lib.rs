use wasm_bindgen::prelude::*;

pub mod lexer;
pub mod parser;
pub mod syntax;
pub mod tokenizer;
pub mod utils;
pub mod walker;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, csstree-wasm!");
}
