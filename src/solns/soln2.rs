use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;
use rayon::slice::ParallelSliceMut;

use crate::solns::Temperature;

pub fn soln2() {
    let start = std::time::Instant::now();
    let chunk_size = 1_000_000;
    let file = File::open("measurements.txt").expect("Failed opening file");
    let lines = BufReader::new(file).lines().chunks(chunk_size);

    let mut records = HashMap::<String, Temperature>::with_capacity(10_000);
    lines.into_iter().for_each(|chunks| {
        for line in chunks {
            let line = line.unwrap();
            let (name, temp) = line.split_once(';').unwrap();
            let temp = temp.parse::<f32>().unwrap();
            if let Some(t) = records.get_mut(name) {
                t.sum += temp;
                t.count += 1;
                if t.min > temp {
                    t.min = temp;
                    return;
                }
                if t.max < temp {
                    t.max = temp;
                }
            } else {
                records.insert(name.to_string(), Temperature::new(temp, temp, temp));
            }
        }
    });

    let mut keys = records.keys().collect::<Vec<_>>();
    keys.par_sort_unstable();
    for key in keys {
        let t = records.get(key).unwrap();
        println!("{}={:.1}/{:.1}/{:.1}", key, t.min, t.mean(), t.max);
    }
    println!("\nsoln2: {:?}", start.elapsed());
    // std::process::exit(0); // comment out this line to benchmark
}
