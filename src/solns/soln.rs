use crate::{parse_decimal_to_integer_optimized, solns::Temperature};
use memchr::{memchr, memchr_iter};
use memmap::{Mmap, MmapOptions};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    fs::File,
    io::{self, Write},
    sync::Arc,
};

use super::Name;
use fxhash::FxHashMap;
const UNIQUE_STATIONS: usize = 512; // although there are only 413 unique stations, we can use 512 to reduce any potential collisions

pub fn soln() {
    let start = std::time::Instant::now();
    let file = File::open("measurements_1b.txt").expect("Failed opening file");
    let mmap = unsafe { MmapOptions::new().map(&file).expect("oops") };
    let data: Arc<Mmap> = Arc::new(mmap);
    // let num_threads = 8; // only want to use 8.
    let num_threads = num_cpus::get();
    println!("Number of threads: {}", num_threads);
    let positions = split_file(num_threads, &data);

    let thread_data = (0..positions.len() - 1)
        .into_par_iter()
        .map(|i| {
            let start = positions[i];
            let end = positions[i + 1];
            process(start, end, Arc::clone(&data))
        })
        .collect();

    println!("time taken for processing: {:?}", start.elapsed());
    let start_merge = std::time::Instant::now();
    let mut results = merge_hashmaps(thread_data)
        .into_values()
        .collect::<Vec<_>>();
    results.sort_unstable_by_key(|t| t.name.clone());
    let mut buffer = String::new();
    results.into_iter().for_each(|t: Temperature| {
        let name = unsafe { std::str::from_utf8_unchecked(&t.name) };
        buffer.push_str(&format!(
            "{}={:.1}/{:.1}/{:.1}\n",
            name,
            t.min as f32 / 10.0,
            t.mean() as f32 / 10.0,
            t.max as f32 / 10.0
        ));
    });

    io::stdout()
        .write_all(buffer.as_bytes())
        .expect("Failed to write");
    println!(
        "\nTime taken for merging and printing: {:?}",
        start_merge.elapsed()
    );
    println!("Total time taken: {:?}", start.elapsed());
}

#[inline]
fn merge_hashmaps(thread_data: Vec<FxHashMap<Name, Temperature>>) -> FxHashMap<Name, Temperature> {
    let mut record: FxHashMap<Name, Temperature> =
        FxHashMap::with_capacity_and_hasher(UNIQUE_STATIONS, Default::default());
    for t in thread_data {
        for (key, value) in t {
            if let Some(t) = record.get_mut(&key) {
                t.sum += value.sum;
                t.count += value.count;
                if t.min > value.min {
                    t.min = value.min;
                }
                if t.max < value.max {
                    t.max = value.max;
                }
            } else {
                record.insert(key.to_owned(), Temperature::new(key, value.min));
            }
        }
    }
    record
}

#[inline]
fn split_file(num_of_threads: usize, data: &[u8]) -> Vec<usize> {
    let mut split_points: Vec<usize> = Vec::with_capacity(num_of_threads);
    let chunk_size = data.len() / num_of_threads;
    let mut start = 0;
    for _ in 1..num_of_threads {
        // Calculate an approximate starting point for the current thread
        let next_split = start + chunk_size;
        // Find the nearest newline after the approximate split point
        if let Some(newline_pos) = memchr(b'\n', &data[next_split..]) {
            // Adjust to absolute position in `data` and move past the newline character
            start = next_split + newline_pos + 1;
            split_points.push(start);
        } else {
            // If no newline found, stop splitting and use the remaining data
            break;
        }
    }
    // Always include the end of the file as the last split point
    split_points.push(data.len());
    split_points
}

#[inline]
fn process(start: usize, end: usize, data: Arc<Mmap>) -> FxHashMap<Name, Temperature> {
    let data = &data[start..end];
    let mut record: FxHashMap<Name, Temperature> =
        FxHashMap::with_capacity_and_hasher(UNIQUE_STATIONS, Default::default());
    let mut last_pos = 0;
    for next_pos in memchr_iter(b'\n', data) {
        let line = &data[last_pos..next_pos];
        last_pos = next_pos + 1;

        let mut split = line.split(|&c| c == b';');
        let (name, temp) = (split.next().unwrap(), split.next().unwrap());
        let temp = parse_decimal_to_integer_optimized(temp);
        if let Some(t) = record.get_mut(name) {
            t.sum += temp;
            t.count += 1;
            if t.min > temp {
                t.min = temp;
            } else if t.max < temp {
                t.max = temp;
            }
        } else {
            record.insert(name.to_vec(), Temperature::new(name.to_vec(), temp));
        }
    }
    record
}
