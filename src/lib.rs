extern crate itertools;
extern crate rayon;

use itertools::Itertools;
use rayon::prelude::*;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DistanceMatrix {
    pub size: usize,
    pub array: Vec<Option<u32>>,
}

impl DistanceMatrix {
    /// Creates new Distance Matrix
    pub fn new(n: usize, distances: Vec<u32>) -> DistanceMatrix {
        let new_array: Vec<Option<u32>> = distances
            .iter()
            .map(|i| match i {
                &0 => None,
                x => Some(*x),
            })
            .collect();
        let new_dist = DistanceMatrix {
            array: new_array,
            size: n,
        };
        assert_eq!(
            new_dist.array.len(),
            new_dist.size * new_dist.size,
            "Array and size do not match"
        );
        new_dist
    }
    /// Gets distance from v1 to v2
    pub fn get(&self, v1: usize, v2: usize) -> Option<u32> {
        self.array[v2 + self.size * v1]
    }
}

/// Traveling Salesperson Problem
///
// this is the primary function
pub fn travel(w_array: DistanceMatrix) -> Vec<usize> {
    // create the hash map
    let mut dist_map: HashMap<Vec<usize>, Vec<(usize, Option<u32>)>> = HashMap::new();
    //   calculate the void column
    let void_col: Vec<(usize, Option<u32>)> = (0..w_array.size)
        .map(|i: usize| (0, w_array.get(i, 0)))
        .collect();
    // println!("[], {:?}", void_col);
    // add empty set column to distance map
    dist_map.insert(Vec::new(), void_col);
    // for each combination size
    for set_len in 1..(w_array.size - 1) {
        // combinations of length set_len
        let comb_by_len = (1..w_array.size).combinations(set_len).collect_vec();
        //        println!("{:?}", comb_by_len);
        // iterate through all combinations of a given length
        let new_cols: Vec<(Vec<usize>, Vec<(usize, Option<u32>)>)> = comb_by_len
            .iter()
            .map(|comb| {
            let mut dist: Vec<(usize, Option<u32>)> = vec![(0, None); w_array.size];
            // for each possible value
            (1..w_array.size)
                // filter out all values if it's contained in the the combination
                .filter(|i| !comb.contains(i))
                .for_each(|i| {
                    let vals: Vec<(usize, Option<u32>)> = comb
                        .iter()
                        .map(|k| {
                            let w = w_array.get(i, *k);
                            // gets the correct column for A - Vx
                            let d_col: Vec<usize> =
                                comb.iter().filter(|x| *x != k).cloned().collect();
                            // gets the value of D[Vx][A - Vx]
                            let d_val: Option<u32> = dist_map[&d_col][*k].1;
                            // add the two together
                            let val: Option<u32> = match (w, d_val) {
                                (Some(x), Some(y)) => Some(x + y), // if both are not infinity, add them
                                (_, _) => None, // else (at least one is infinity, retrn none
                            };
                            (*k, val)
                        })
                        .collect();
                    // Find the minimum value that is not None
                    let min_val: Option<&(usize, Option<u32>)> =
                        vals.iter().filter(|i| i.1.is_some()).min_by_key(|i| i.1);
                    dist[i] = match min_val {
                        None => (0, None),
                        Some(x) => *x,
                    };
                });
            // println!("{:?}, {:?}", comb, dist);
            // dist_map.insert(comb.clone(), dist);
            (comb.clone(), dist)
        }).collect();
        new_cols.iter().for_each(|i| {
            dist_map.insert(i.0.clone(), i.1.clone());
        });
    }
    let mut comb: Vec<usize> = (1..w_array.size).collect();
    let vals: Vec<(usize, Option<u32>)> = comb
        .iter()
        .map(|k| {
            let w = w_array.get(0, *k);
            // gets the correct column for A - Vx
            let d_col: Vec<usize> = comb.iter().filter(|x| *x != k).cloned().collect();
            // gets the value of D[Vx][A - Vx]
            let d_val: Option<u32> = dist_map[&d_col][*k].1;
            // add the two together
            let val: Option<u32> = match (w, d_val) {
                (Some(x), Some(y)) => Some(x + y), // if both are not infinity, add them
                (_, _) => None,                    // else (at least one is infinity, retrn none
            };
            (*k, val)
        })
        .collect();
    let last_val: (usize, Option<u32>) = *vals
        .iter()
        .filter(|i| i.1.is_some())
        .min_by_key(|i| i.1)
        .unwrap();
    let mut trip: Vec<usize> = vec![0];
    // calculate minimum trip
    // set next city
    let mut next: usize = last_val.0;
    trip.push(next);
    comb = comb.iter().filter(|i| **i != next).cloned().collect();
    while !comb.is_empty() {
        next = dist_map[&comb][next].0;
        trip.push(next);
        comb = comb.iter().filter(|i| **i != next).cloned().collect();
    }
    trip.push(0);
    // let last_comb: Vec<usize> = (1..w_array.size).collect();
    // println!("{:?}: [({}, {:?})]", last_comb, last_val.0, last_val.1);
    // println!("Trip: {:?}", trip);
    // println!("Distance: {}", last_val.1.unwrap());
    trip
}

pub fn par_travel(w_array: DistanceMatrix) -> Vec<usize> {
    // create the hash map
    let mut dist_map: HashMap<Vec<usize>, Vec<(usize, Option<u32>)>> = HashMap::new();
    //   calculate the void column
    let void_col: Vec<(usize, Option<u32>)> = (0..w_array.size)
        .map(|i: usize| (0, w_array.get(i, 0)))
        .collect();
    // println!("[], {:?}", void_col);
    // add empty set column to distance map
    dist_map.insert(Vec::new(), void_col);
    // for each combination size
    for set_len in 1..(w_array.size - 1) {
        // combinations of length set_len
        let comb_by_len = (1..w_array.size).combinations(set_len).collect_vec();
        //        println!("{:?}", comb_by_len);
        // iterate through all combinations of a given length
        let new_cols: Vec<(Vec<usize>, Vec<(usize, Option<u32>)>)> = comb_by_len
            .par_iter()
            .map(|comb| {
            let mut dist: Vec<(usize, Option<u32>)> = vec![(0, None); w_array.size];
            // for each possible value
            (1..w_array.size)
                // filter out all values if it's contained in the the combination
                .filter(|i| !comb.contains(i))
                .for_each(|i| {
                    let vals: Vec<(usize, Option<u32>)> = comb
                        .iter()
                        .map(|k| {
                            let w = w_array.get(i, *k);
                            // gets the correct column for A - Vx
                            let d_col: Vec<usize> =
                                comb.iter().filter(|x| *x != k).cloned().collect();
                            // gets the value of D[Vx][A - Vx]
                            let d_val: Option<u32> = dist_map[&d_col][*k].1;
                            // add the two together
                            let val: Option<u32> = match (w, d_val) {
                                (Some(x), Some(y)) => Some(x + y), // if both are not infinity, add them
                                (_, _) => None, // else (at least one is infinity, retrn none
                            };
                            (*k, val)
                        })
                        .collect();
                    // Find the minimum value that is not None
                    let min_val: Option<&(usize, Option<u32>)> =
                        vals.iter().filter(|i| i.1.is_some()).min_by_key(|i| i.1);
                    dist[i] = match min_val {
                        None => (0, None),
                        Some(x) => *x,
                    };
                });
            // println!("{:?}, {:?}", comb, dist);
            // dist_map.insert(comb.clone(), dist);
            (comb.clone(), dist)
        }).collect();
        new_cols.iter().for_each(|i| {
            dist_map.insert(i.0.clone(), i.1.clone());
        });
    }
    let mut comb: Vec<usize> = (1..w_array.size).collect();
    let vals: Vec<(usize, Option<u32>)> = comb
        .iter()
        .map(|k| {
            let w = w_array.get(0, *k);
            // gets the correct column for A - Vx
            let d_col: Vec<usize> = comb.iter().filter(|x| *x != k).cloned().collect();
            // gets the value of D[Vx][A - Vx]
            let d_val: Option<u32> = dist_map[&d_col][*k].1;
            // add the two together
            let val: Option<u32> = match (w, d_val) {
                (Some(x), Some(y)) => Some(x + y), // if both are not infinity, add them
                (_, _) => None,                    // else (at least one is infinity, retrn none
            };
            (*k, val)
        })
        .collect();
    let last_val: (usize, Option<u32>) = *vals
        .iter()
        .filter(|i| i.1.is_some())
        .min_by_key(|i| i.1)
        .unwrap();
    let mut trip: Vec<usize> = vec![0];
    // calculate minimum trip
    // set next city
    let mut next: usize = last_val.0;
    trip.push(next);
    comb = comb.iter().filter(|i| **i != next).cloned().collect();
    while !comb.is_empty() {
        next = dist_map[&comb][next].0;
        trip.push(next);
        comb = comb.iter().filter(|i| **i != next).cloned().collect();
    }
    trip.push(0);
    // let last_comb: Vec<usize> = (1..w_array.size).collect();
    // println!("{:?}: [({}, {:?})]", last_comb, last_val.0, last_val.1);
    // println!("Trip: {:?}", trip);
    // println!("Distance: {}", last_val.1.unwrap());
    trip
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_d() {
        let distances: Vec<u32> = vec![
            0, 2, 9, 0, 
            1, 0, 6, 4, 
            0, 7, 0, 8, 
            6, 3, 0, 0
        ];
        let w: DistanceMatrix = DistanceMatrix::new(4, distances);
        let result = travel(w);
        assert_eq!(result, vec![0, 2, 3, 1, 0]);
    }

    #[test]
    fn five_d() {
        let distances: Vec<u32> = vec![
            0, 1, 0, 1, 5, 
            9, 0, 3, 2, 0, 
            0, 0, 0, 4, 0, 
            0, 0, 2, 0, 3, 
            3, 0, 0, 0, 0,
        ];
        let w: DistanceMatrix = DistanceMatrix::new(5, distances);
        let result = travel(w);
        assert_eq!(result, vec![0, 1, 2, 3, 4, 0]);
    }

    #[test]
    #[should_panic]
    fn invalid_distance_matrix_1() {
        let d: Vec<u32> = vec![0, 1];
        let _w: DistanceMatrix = DistanceMatrix::new(2, d);
    }

    #[test]
    #[should_panic]
    fn invalid_distance_get() {
        let d: Vec<u32> = vec![
            0, 1,
            2, 0
        ];
        let w: DistanceMatrix = DistanceMatrix::new(2, d);
        let _x = w.get(2, 0);
    }
}
