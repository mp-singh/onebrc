use crate::{parse_decimal_to_integer_optimized, solns::Temperature};
use memchr::{memchr, memchr_iter};
use memmap::{Mmap, MmapOptions};
use std::{fs::File, sync::Arc};

use super::Name;
use fxhash::FxHashMap;

pub fn soln() {
    let start = std::time::Instant::now();
    let file = File::open("measurements_1b.txt").expect("Failed opening file");
    let mmap = unsafe { MmapOptions::new().map(&file).expect("oops") };
    let data: Arc<Mmap> = Arc::new(mmap);
    let num_threads = 8; // only want to use 8.
    println!("Number of threads: {}", num_threads);
    let positions = split_file(num_threads, &data);

    let threads = (0..positions.len())
        .map(|i| {
            let data = Arc::clone(&data);
            let start = positions[i];
            let end = positions.get(i + 1).cloned().unwrap_or(data.len());
            std::thread::spawn(move || process(start, end, data))
        })
        .collect::<Vec<_>>();

    let thread_data = threads
        .into_iter()
        .map(|t| t.join().unwrap())
        .collect::<Vec<_>>();

    println!("time taken for processing: {:?}", start.elapsed());
    let start_merge = std::time::Instant::now();
    let mut results = merge_hashmaps(thread_data)
        .into_values()
        .collect::<Vec<_>>();
    results.sort_by_key(|t| t.name.clone());

    results.into_iter().enumerate().for_each(|(_, t)| {
        let name = unsafe { std::str::from_utf8_unchecked(&t.name) };
        println!(
            "{}={}/{}/{}",
            name,
            t.min as f32 / 10.0,
            t.mean() as f32 / 10.0,
            t.max as f32 / 10.0
        );
    });
    println!(
        "\nTime taken for merging and printing: {:?}",
        start_merge.elapsed()
    );
    println!("Total time taken: {:?}", start.elapsed());
}

fn merge_hashmaps(thread_data: Vec<FxHashMap<Name, Temperature>>) -> FxHashMap<Name, Temperature> {
    // let mut hashmap = FxHashMap::default();
    let mut record: FxHashMap<Name, Temperature> =
        FxHashMap::with_capacity_and_hasher(10_000, Default::default());

    for t in thread_data {
        for (key, value) in t {
            let t = record
                .entry(key.clone())
                .or_insert(Temperature::new(key, value.min));
            t.sum += value.sum;
            t.count += value.count;
            if t.min > value.min {
                t.min = value.min;
            }
            if t.max < value.max {
                t.max = value.max;
            }
        }
    }
    record
}
fn split_file(num_of_threads: usize, data: &[u8]) -> Vec<usize> {
    let mut split_points: Vec<usize> = Vec::new();
    for i in 0..num_of_threads {
        let start = data.len() / num_of_threads * i;
        let newline = memchr(b'\n', &data[start..]).unwrap();
        split_points.push(start + newline + 1);
    }
    split_points
}

fn process(start: usize, end: usize, data: Arc<Mmap>) -> FxHashMap<Name, Temperature> {
    let data = &data[start..end];
    let mut record: FxHashMap<Name, Temperature> =
        FxHashMap::with_capacity_and_hasher(1000, Default::default());
    let mut last_pos = 0;
    for next_pos in memchr_iter(b'\n', data) {
        let line = &data[last_pos..next_pos];
        last_pos = next_pos + 1;

        let line = unsafe { std::str::from_utf8_unchecked(line) };
        let (name, temp) = line.split_once(';').unwrap();
        let temp = parse_decimal_to_integer_optimized(temp);
        let t = record
            .entry(name.as_bytes().to_vec())
            .or_insert(Temperature::new(name.into(), temp));
        t.update(temp);
    }
    record
}
