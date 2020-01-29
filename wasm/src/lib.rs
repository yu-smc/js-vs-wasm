extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn (s: f64);
}

#[wasm_bindgen]
pub fn greet(name: f64) -> f64{
    name / 2.0
}

