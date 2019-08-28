extern crate wasm_bindgen;
extern crate web_sys;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct ClusterJson {
  x: f64,
  y: f64,
  weight: i32
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
  ( $( $t:tt )* ) => {
    web_sys::console::log_1(&format!( $( $t )* ).into());
  }
}

use wasm_bindgen::prelude::*;
mod fuzzy_distance_cluster;
mod fuzzy_point_map;
use std::vec::Vec;
use std::collections::{HashMap, BinaryHeap};
use crate::fuzzy_point_map::FuzzyPointMap;
use crate::fuzzy_distance_cluster::FuzzyDistanceCluster;

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

#[wasm_bindgen]
pub fn cluster(x_arr: Vec<f64>, y_arr: Vec<f64>, w_arr: Vec<f64>, distance: f64) -> String {

  // log!("x: {:?}, y: {:?}, w: {:?}", x_arr, y_arr, w_arr);
  let v1 = x_arr.iter()
    .zip(y_arr.iter())
    .zip(w_arr.iter())
    .map(|((x, y), w)| {
      fuzzy_point_map::FuzzyPoint{x: *x, y: *y, weight: *w}
    })
    .collect::<Vec<fuzzy_point_map::FuzzyPoint>>();

  // log!("{:?}", v1);

  let indexed_vec = v1.iter().enumerate()
    .map(|(index, &fzm)| (fzm, index as i32))
    .collect::<Vec<(fuzzy_point_map::FuzzyPoint, i32)>>();

  // log!("indexed vec {:?}", indexed_vec);
  let fuzzy_cluster_app = fuzzy_distance_cluster::FuzzyDistanceClusterClass{distance: distance, grid_size: distance * 0.09};

  let clusters = fuzzy_cluster_app.apply(indexed_vec);
  let clusters_as_json = clusters.iter()
    .map(|&((x, y), w)| {
      let result_object = ClusterJson{x: x, y: y, weight: w};
      serde_json::to_string(&result_object).unwrap()
    })
    .collect::<Vec<String>>();
  format!("{:?}", clusters_as_json).into()
}