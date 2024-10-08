extern crate wasm_bindgen;
mod core;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_drf(s: &str) -> String {
    let mut parser = crate::core::parser::drf::DrfParser::from_string(s);
    parser.parse();
    let s = serde_json::to_string(&parser.drf).unwrap();
    s
}

#[wasm_bindgen]
pub fn parse_layermap(s: &str) -> String {
    let mut parser = crate::core::parser::layermap::LayerMapParser::from_string(s);
    parser.parse();
    parser.to_json()
}
