use std::collections::{HashMap, HashSet, BinaryHeap};
use std::vec::Vec;
use super::fuzzy_point_map;
use crate::fuzzy_point_map::{FuzzyPoint, FuzzyPointMap, FuzzyPointMapClass};

pub struct FuzzyDistanceClusterClass {
  pub distance: f64,
  pub grid_size: f64
}

pub trait FuzzyDistanceCluster {
  fn apply(&self, v1: Vec<(FuzzyPoint, i32)>) -> Vec<((f64, f64), i32)>;
  fn next_repr(&self, acc: Vec<((f64, f64), Vec<(f64, f64)>)>, unpicked_set: HashSet<(i32, i32)>, fuzzy_points_map: &mut FuzzyPointMapClass, pq: &mut BinaryHeap<(i32, (i32, i32))>) -> Vec<((f64, f64), Vec<(f64, f64)>)>;
}

impl FuzzyDistanceCluster for FuzzyDistanceClusterClass {
  fn apply(&self, v1: Vec<(FuzzyPoint, i32)>) -> Vec<((f64, f64), i32)> {
    let pts: Vec<((f64, f64), i32)> = v1
      .iter()
      .map(|&(p, idx)| ((p.x, p.y), idx))
      .collect();

    let fuzzy_points_map = fuzzy_point_map::from_seq(&pts, self.grid_size);
    println!("fpm: {:?}", fuzzy_points_map);
    let mut grid = fuzzy_point_map::FuzzyPointMapClass {map: fuzzy_points_map, grid_size: self.grid_size};


    let neighbour_counts_vec: Vec<((i32, i32), i32)> = grid.map.iter()
      .map(|(&key, _)| {
        let center = fuzzy_point_map::grid_center(self.grid_size)(key);
        let cells_in_range = grid.cells(
          fuzzy_point_map::FuzzyPoint{x: center.0, y: center.1, weight: 1.0},
          self.distance
        );
        println!("{:?}", center);
        // println!("{:?}", cells_in_range);
        println!();
        let num_neighbours: usize = cells_in_range.iter().map(|uv| {
          match grid.map.get(&uv) {
            Some(r) => r.len(),
            None => 0
          }
        }).sum();
        // .collect(); //.sum

        (key, num_neighbours as i32)
      })
      .collect();
    let neighbour_counts: HashMap<_, _> = neighbour_counts_vec.into_iter().collect();

    let initial_neighbours: HashSet<(i32, (i32, i32))> = pts.iter().map(|&(xy, _)| {
      let uv = fuzzy_point_map::key_pair_fn(self.grid_size)(xy);
      (
        neighbour_counts.get(&uv).map_or(0, |&xy| xy),
        uv
      )
    }).collect();

    let mut pq: BinaryHeap<(i32, (i32, i32))> = BinaryHeap::new();
    for neighbour in initial_neighbours {
      pq.push(neighbour);
    }
    let mut acc: Vec<((f64, f64), Vec<(f64, f64)>)> = Vec::new();
    let key_set: HashSet<(i32, i32)> = fuzzy_point_map::from_seq(&pts, self.grid_size).iter().map(|(&k, _)| k).collect();

    // println!("key set {:?}", key_set);
    let mut clusters = self.next_repr(acc, key_set, &mut grid, &mut pq);

    clusters
      .iter()
      .cloned()
      .map(|(cluster, points)| {
        (cluster, points.len() as i32)
      }).collect::<Vec<((f64, f64), i32)>>()

  }

  fn next_repr (&self, mut acc: Vec<((f64, f64), Vec<(f64, f64)>)>,
    unpicked_set: HashSet<(i32, i32)>,
    fuzzy_points_map: &mut FuzzyPointMapClass,
    pq: &mut BinaryHeap<(i32, (i32, i32))>)-> Vec<((f64, f64), Vec<(f64, f64)>)> {
    if pq.peek() == None {
      acc
    } else {
      let (score, uv) = match pq.pop() {
        Some((s, uv)) => (s, uv),
        None => panic!("pq empty")
      };

      let xy = fuzzy_point_map::grid_center(self.grid_size)(uv);
      let neighbouring_grid_centres: Vec<(i32, i32)> = fuzzy_points_map.cells(FuzzyPoint{x: xy.0, y: xy.1, weight: 1.0}, self.distance);
      let actual_score: usize = neighbouring_grid_centres
        .iter()
        .map(|c| {
          match fuzzy_points_map.map.get(c) {
            Some(s) => s.len(),
            None => 0
          }
        }).sum();

      // println!("actual score {}, score {}", actual_score, score);

      if !unpicked_set.contains(&uv) {
        return self.next_repr(acc, unpicked_set, fuzzy_points_map, pq)
      } else if actual_score as i32 == score {
        let prepend_acc = neighbouring_grid_centres
          .iter()
          .flat_map(|c| {
            match fuzzy_points_map.map.get(c) {
              Some(v) => v,
              None => panic!("Not found in flatmap (this usually means candidate keys are not filtered properly)")
            }
          })
          .map(|&(kv, idx)| (kv.0 as f64, kv.1 as f64))
          .collect::<Vec<(f64, f64)>>();

        let mut pre = vec![(xy, prepend_acc)];
        acc.append(&mut pre);

        let neighbouring_grid_centers_set = neighbouring_grid_centres
          .iter()
          .map(|&v| v) // needed to clone
          .collect::<HashSet<(i32, i32)>>();

        let new_unpicked = unpicked_set
          .difference(&neighbouring_grid_centers_set)
          .map(|&v| v) // needed to clone
          .collect::<HashSet<_>>();

        // println!("neighbouring {:?} \n unpicked {:?}", neighbouring_grid_centers_set, new_unpicked);
        // HashMap<(i32, i32), HashSet<(i32, i32)>>
        // let points_to_remove = fuzzy_points_map.map
        //   .iter()
        //   .filter(|(k, _)| {
        //     neighbouring_grid_centers_set.contains(k)
        //   });

        for point in neighbouring_grid_centers_set.iter() {
          fuzzy_points_map.map.remove(&point);
        }

        return self.next_repr(
          acc,
          new_unpicked,
          fuzzy_points_map,
          pq
        );
      } else if (actual_score as i32) < score {
        if actual_score as i32 > 0 {
          pq.push((actual_score as i32, uv));
        }
        return self.next_repr(acc, unpicked_set, fuzzy_points_map, pq)
      } else {
        panic!("illegal state")
      }
    }
  }
}
