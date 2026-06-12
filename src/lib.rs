extern crate wasm_bindgen;
pub mod core;

use std::error::Error;

pub fn parse_drf_file(path: &str) -> Result<String, Box<dyn Error>> {
    let mut parser = core::parser::drf::DrfParser::from_path(path)?;
    parser.parse();
    Ok(serde_json::to_string(&parser.drf)?)
}

pub fn parse_layermap_file(path: &str) -> Result<String, Box<dyn Error>> {
    let mut parser = core::parser::layermap::LayerMapParser::from_path(path)?;
    parser.parse();
    Ok(parser.to_json())
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn parse_drf(s: &str) -> String {
    let mut parser = crate::core::parser::drf::DrfParser::from_string(s);
    parser.parse();
    serde_json::to_string(&parser.drf).unwrap()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn parse_layermap(s: &str) -> String {
    let mut parser = crate::core::parser::layermap::LayerMapParser::from_string(s);
    parser.parse();
    parser.to_json()
}
