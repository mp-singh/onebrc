// use std::io::Read;
// use std::sync::Mutex;
// use std::{
//     collections::HashMap,
//     fs::File,
//     io::{BufRead, BufReader},
// };

// use itertools::Itertools;
// use memmap::Mmap;
// use rayon::iter::ParallelBridge;
// use rayon::vec;
// use rayon::{iter::IntoParallelRefIterator, slice::ParallelSliceMut};

use crate::{parse_decimal_to_integer_optimized, solns::Temperature};

// pub fn soln3() {
//     let start = std::time::Instant::now();
//     let chunk_size = 1_000_000;
//     let file = File::open("measurements_1b.txt").unwrap();
//     let mmap = unsafe { Mmap::map(&file).unwrap() };
//     // let mut count = 0;
//     // for _ in mmap.split(|c| *c as char == '\n') {
//     //     //println!("{}", line);
//     //     count += 1;
//     // }
//     // println!("{:?}", start.elapsed());

//     let lines = mmap.lines().chunks(chunk_size);

//     let mut records = HashMap::<String, Temperature>::with_capacity(10_000);
//     lines.into_iter().for_each(|chunks| {
//         for line in chunks {
//             let line = line.unwrap();
//             let (name, temp) = line.split_once(';').unwrap();
//             let temp = parse_decimal_to_integer_optimized(temp);
//             if let Some(t) = records.get_mut(name) {
//                 t.sum += temp;
//                 t.count += 1;
//                 if t.min > temp {
//                     t.min = temp;
//                     return;
//                 }
//                 if t.max < temp {
//                     t.max = temp;
//                 }
//             } else {
//                 records.insert(name.to_string(), Temperature::new(temp));
//             }
//         }
//     });

//     let mut keys = records.keys().collect::<Vec<_>>();
//     keys.par_sort_unstable();
//     for key in keys {
//         let t = records.get(key).unwrap();
//         println!(
//             "{}={}/{}/{}",
//             key,
//             t.min as f32 / 10.0,
//             t.mean() as f32 / 10.0,
//             t.max as f32 / 10.0
//         );
//     }
//     println!("\nsoln3: {:?}", start.elapsed());
//     // std::process::exit(0); // comment out this line to benchmark
// }

use std::{
    collections::HashMap,
    fs::File,
    sync::{
        atomic::{self, AtomicUsize},
        Arc,
    },
};

use memchr::{memchr, memchr_iter};
use memmap::{Mmap, MmapOptions};

pub fn soln() {
    let start = std::time::Instant::now();
    let file = File::open("measurements_1b.txt").expect("Failed opening file");
    let mmap = unsafe { MmapOptions::new().map(&file).expect("oops") };
    let data: Arc<Mmap> = Arc::new(mmap);
    let num_threads = 8;
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
    // measure time taken to merge the hashmaps
    let start_merge = std::time::Instant::now();
    merge_hashmaps(thread_data);
    println!("time taken for merging: {:?}", start_merge.elapsed());
    println!("Time taken: {:?}", start.elapsed());
}

fn merge_hashmaps(thread_data: Vec<HashMap<String, Temperature>>) -> HashMap<String, Temperature> {
    let mut record: HashMap<String, Temperature> = HashMap::with_capacity(10_000);
    for t in thread_data {
        for (k, v) in t {
            if let Some(t) = record.get_mut(&k) {
                t.sum += v.sum;
                t.count += v.count;
                if t.min > v.min {
                    t.min = v.min;
                    continue;
                }
                if t.max < v.max {
                    t.max = v.max;
                }
            } else {
                record.insert(k, v);
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

fn process(start: usize, end: usize, data: Arc<Mmap>) -> HashMap<String, Temperature> {
    let data = &data[start..end];
    let mut record: HashMap<String, Temperature> = HashMap::new();
    let mut last_pos = 0;
    for next_pos in memchr_iter(b'\n', data) {
        let line = &data[last_pos..next_pos];
        last_pos = next_pos + 1;

        let line = std::str::from_utf8(line).unwrap();
        let (name, temp) = line.split_once(';').unwrap();
        let temp = parse_decimal_to_integer_optimized(temp);
        if let Some(t) = record.get_mut(name) {
            t.sum += temp;
            t.count += 1;
            if t.min > temp {
                t.min = temp;
                continue;
            }
            if t.max < temp {
                t.max = temp;
            }
        } else {
            record.insert(name.to_string(), Temperature::new(temp));
        }
    }
    record
}
