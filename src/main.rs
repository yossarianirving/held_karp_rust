mod lib;
extern crate rand;
use rand::Rng;
use std::time::Instant;
fn main() {
    let distances: Vec<u32> = vec![0, 2, 9, 0, 1, 0, 6, 4, 0, 7, 0, 8, 6, 3, 0, 0];
    let w: lib::DistanceMatrix = lib::DistanceMatrix::new(4, distances);
    lib::travel(w);
    let distances2: Vec<u32> = vec![
        0, 1, 0, 1, 5, 9, 0, 3, 2, 0, 0, 0, 0, 4, 0, 0, 0, 2, 0, 3, 3, 0, 0, 0, 0,
    ];
    let w2: lib::DistanceMatrix = lib::DistanceMatrix::new(5, distances2);
    lib::travel(w2);
    let n: usize = 20;
    let mut rng = rand::thread_rng();
    let distances3: Vec<u32> = (0..(n * n)).map(|_| {
        rng.gen_range(0, 18)
    }).collect();
    let w3: lib::DistanceMatrix = lib::DistanceMatrix::new(n, distances3);
    let mut now = Instant::now();
    lib::travel(w3.clone());
    println!("Non-parallel {}: {}ms", n, now.elapsed().as_millis());
    now = Instant::now();
    lib::par_travel(w3);
    println!("Parallel {}: {}ms", n, now.elapsed().as_millis());

}
