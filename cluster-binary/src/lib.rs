extern crate wasm_bindgen;
extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
  ( $( $t:tt )* ) => {
    web_sys::console::log_1(&format!( $( $t )* ).into());
  }
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
  pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
  alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn add(int: u32) -> u32 {
  return (2 + int) as u32;
}

// #[wasm_bindgen]
// pub fn cluster(float: f32) {

// }