use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::borrow::Cow;

#[derive(Debug, Copy, Clone)]
pub struct FuzzyPoint {
  pub x: f64,
  pub y: f64,
  pub weight: f64
}

pub trait FuzzyPointMap {
  fn cells(&self, p: FuzzyPoint, r: f64) -> Vec<(i32, i32)>;
  fn count_range(&self, point: FuzzyPoint, r: f64) -> i32;
  // fn query_range(&self, point: FuzzyPoint, r: f64) -> Vec<&(i32, i32)>;
  // fn from_seq(&self, points: Vec<(f64, f64)>, grid: f64) -> HashMap<(i32, i32), HashSet<(i32, i32)>>;
}

pub struct FuzzyPointMapClass {
  // map: Map[(Int, Int), Set[((Double, Double), A)]],
  //                           gridSize: Double
  // till we find a workaround for set<f64> we will have to lose precision
  pub map: HashMap<(i32, i32), HashSet<((i32, i32), i32)>>,
  pub grid_size: f64
}

impl FuzzyPointMap for FuzzyPointMapClass {
  fn cells(&self, point: FuzzyPoint, r: f64) -> Vec<(i32, i32)> {
    let minx = key_fn(self.grid_size)(point.x - r);
    let maxx = key_fn(self.grid_size)(point.x + r);
    let miny = key_fn(self.grid_size)(point.y - r);
    let maxy = key_fn(self.grid_size)(point.y + r);

    // println!("HIIII {:?} {:?} {:?} {:?}", minx, maxx, miny, maxy);

    let candidate_keys = |minx: i32, maxx: i32, miny: i32, maxy: i32| {
      let mut vec: Vec<(i32, i32)> = vec![];
      for i in minx..maxx {
        for j in miny..maxy {
          vec.push((i, j));
        }
      }
      return vec;
    };

    // println!("CANDIDATE KEYS {:?}", candidate_keys(minx, maxx, miny, maxy));

    let all_included = |key: (f64, f64)| -> bool {
      let other_point = FuzzyPoint{x: key.0 * self.grid_size, y: key.1 * self.grid_size, weight: 1.0};
      return within_range(&point, &other_point, r as f64);
    };

    return candidate_keys(minx, maxx, miny, maxy)
      .iter()
      .filter(|&xy| self.map.contains_key(xy))
      .filter(|(x, y)| all_included((*x as f64, *y as f64)))
      .map(|(x, y)| (*x as i32, *y as i32))
      .collect();

  }

  fn count_range(&self, point: FuzzyPoint, r: f64) -> i32 {
    let full_cells = self.cells(point, r);
    full_cells.iter().map(|&k| {
      match self.map.get(&k) {
        Some(set) => {
          let ys: Vec<i32> = set.iter().map(|&(_, y)| y).collect();
          ys.len() as i32
        },
        None => 0 as i32
      }
    }).sum()
  }

  // def queryRange(point: Point, r: Double) = {
  //   val fullCells = this.cells(point, r)

  //   fullCells.view.flatMap({k => map(k)})
  // }

  // fn query_range(&self, point: FuzzyPoint, r: f64) -> Vec<&(i32, i32)> {
  //   // let vec = vec![(0.0, 0.0)];
  //   let full_cells = self.cells(point, r);
  //   full_cells.iter().flat_map(|k: &(i32, i32)| {
  //     self.map.get(k)
  //       .map(|kvi: ((i32, i32), i32)| (kvi.0))
  //   }).collect()
    // return vec;
  // }
}


pub fn from_seq(points: &Vec<((f64, f64), i32)>, grid_size: f64) -> HashMap<(i32, i32), HashSet<((i32, i32), i32)>> {
  let group_points = |v: &Vec<((f64, f64), i32)>| {
    let mut grouped: HashMap<(i32, i32), HashSet<((i32, i32), i32)>> = HashMap::new();
    for (pair, index) in v {
      let i_pair = ((pair.0 as i32, pair.1 as i32), *index);
      let mapped = (key_fn(grid_size)(pair.0), key_fn(grid_size)(pair.1));
      let p = grouped.entry(mapped);
      match p {
        Entry::Occupied(mut entry) => {
          entry.get_mut().insert(i_pair);
        },
        Entry::Vacant(entry) => {
          let mut hs = HashSet::new();
          hs.insert(i_pair);
          entry.insert(hs);
        }
      }
    }
    // println!("{:?}", grouped);
    grouped
  };
  group_points(points)
}

fn within_range(p1: &FuzzyPoint, p2: &FuzzyPoint, dist: f64) -> bool {
  let dx = p1.x - p2.x;
  let dy = p1.y - p2.y;

  return (dx * dx) + (dy * dy) <= (dist * dist);
}

fn key_fn(grid_size: f64) -> impl Fn(f64) -> i32 {
  move |v| (v / grid_size).round() as i32
}

pub fn key_pair_fn(grid_size: f64) -> impl Fn((f64, f64)) -> (i32, i32) {
  move |(x,y)| (key_fn(grid_size)(x), key_fn(grid_size)(y))
}

pub fn grid_center(grid_size: f64) -> impl Fn((i32, i32)) -> (f64, f64) {
  move |p| (p.0 as f64 * grid_size, p.1 as f64 * grid_size)
}