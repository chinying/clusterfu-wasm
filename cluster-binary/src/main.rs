#![allow(unused_imports)]
#![allow(unused_variables)]

mod call;
mod fuzzy_distance_cluster;
mod fuzzy_point_map;
use std::vec::Vec;
use std::collections::{HashMap, BinaryHeap};
use crate::fuzzy_point_map::FuzzyPointMap;
use crate::fuzzy_distance_cluster::FuzzyDistanceCluster;

fn main() {

  let g_size = 15.0;
  let distance = 100.0;

  let p1 = fuzzy_point_map::FuzzyPoint{x: 10.1, y: 9.9, weight: 1.0};
  let p2 = fuzzy_point_map::FuzzyPoint{x: 17.8, y: 10.0, weight: 1.0};
  let p3 = fuzzy_point_map::FuzzyPoint{x: 10.2, y: 17.8, weight: 1.0};
  let p4 = fuzzy_point_map::FuzzyPoint{x: 32.1, y: 13.2, weight: 1.0};
  let p4_1 = fuzzy_point_map::FuzzyPoint{x: 32.1, y: 13.2, weight: 1.0};

  // cluster(10, &[p1, p2, p3, p4]);
  // let points =
  let v1 = vec![p1, p2, p3, p4, p4_1];
  let indexed_vec = v1.iter().enumerate()
    .map(|(index, &fzm)| (fzm, index as i32))
    .collect::<Vec<(fuzzy_point_map::FuzzyPoint, i32)>>();
  let fuzzy_cluster_app = fuzzy_distance_cluster::FuzzyDistanceClusterClass{distance: distance, grid_size: distance * 0.09};
  fuzzy_cluster_app.apply(indexed_vec);
}