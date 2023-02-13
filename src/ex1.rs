//Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
//and mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

pub fn demo() {
    let mut v1 = vec![0, 3, 2, 1, 0];
    let mut v2 = vec![8, 8, 7, 2];
    println!("median of v1: {}", median(&mut v1));
    println!("median of v2: {}", median(&mut v2));

    println!("mode of v1: {}", mode(&v1));
    println!("mode of v2: {}", mode(&v2));
}

fn median(v: &mut Vec<i32>) -> f32 {
    v.sort();
    let index = v.len() / 2;
    if v.len() % 2 == 0 {
        (v[index] as f32 + v[index - 1] as f32) * 1.0 / 2.0
    } else {
        v[index] as f32
    }
}

fn mode(v: &Vec<i32>) -> i32 {
    let mut hmap = HashMap::new();

    for e in v {
        let count = hmap.entry(*e).or_insert(0);
        *count += 1;
    }

    hmap.into_iter().max_by_key(|e| e.1).unwrap().0
}
