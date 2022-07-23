#![allow(dead_code)]
/*
"Given a list of integers, use a vector and return the
    - mean (the average value)
    - median (when sorted, the value in the middle position)
    - mode (the value that occurs most often; a hash map will be helpful here)
of the list."
*/

use std::collections::HashMap;

pub fn run() {
    let list = [10, 30, 20, 40, 20];
    description(&list);
    mode2(&list);
}

fn description(list: &[i32]) -> (i32, i32, i32) {
    let length = list.len();
    let mut vector = Vec::from(list);
    // MEAN
    let mean = vector.iter().sum::<i32>() / length as i32;
    // MEDIAN
    vector.sort();
    let median = if length % 2 == 0 {
        vector[length / 2 - 1] + vector[length / 2] / 2
    } else {
        vector[length / 2]
    };
    // MODE
    let mode = mode2(&vector);
    println!("MEAN: {}\nMEDIAN: {}\nMODE: {}", mean, median, mode);
    (mean, median, mode)
}

// fn mode(map: &HashMap<i32, i32>) -> i32 {
//     *map.iter().max_by_key(|x| x.1).unwrap().0
// }

fn mode2(vector: &[i32]) -> i32 {
    let mut map = HashMap::new();

    for i in vector.iter() {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    **map.iter().max_by_key(|x| x.1).unwrap().0
}
