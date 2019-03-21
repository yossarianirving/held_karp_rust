mod lib;
fn main() {
    let distances: Vec<u32> = vec![0, 2, 9, 0, 1, 0, 6, 4, 0, 7, 0, 8, 6, 3, 0, 0];
    let w: lib::DistanceMatrix = lib::DistanceMatrix::new(4, distances);
    lib::travel(w);
    let distances2: Vec<u32> = vec![
        0, 1, 0, 1, 5, 9, 0, 3, 2, 0, 0, 0, 0, 4, 0, 0, 0, 2, 0, 3, 3, 0, 0, 0, 0,
    ];
    let w2: lib::DistanceMatrix = lib::DistanceMatrix::new(5, distances2);
    lib::travel(w2);
}
