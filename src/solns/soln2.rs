use std::{
    char,
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader, Cursor, Read, Seek},
    os::unix::process,
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

pub fn soln2a() {
    let start = std::time::Instant::now();
    let file = File::open("measurements.txt").expect("Failed opening file");
    let chunk_size = 1024 * 1024; // 1MB
    let mut buffer = vec![0; chunk_size]; // 1KB buffer
    let mut reader = BufReader::with_capacity(chunk_size * 8, file);
    let mut records = HashMap::<String, Temperature>::with_capacity(10_000);
    let mut offset = 0;
    let mut count = 0;
    loop {
        reader
            .seek(io::SeekFrom::Start(offset))
            .expect("Failed to seek");
        let bytes_read = reader.read(&mut buffer).expect("Failed to read file");
        if bytes_read == 0 {
            break;
        }
        let last_newline = buffer[..bytes_read].iter().rposition(|&x| x == b'\n');

        if let Some(index) = last_newline {
            process_chunk(&buffer[..=index], &mut records, &mut count);
            offset = offset + index as u64 + 1;
        }
    }
    println!("time after loop: {:?}", start.elapsed());
    let mut keys = records.keys().collect::<Vec<_>>();
    keys.par_sort_unstable();
    for key in keys {
        let _t = records.get(key).unwrap();
        // println!("{}={:.1}/{:.1}/{:.1}", key, t.min, t.mean(), t.max);
    }
    println!("\nsoln2: {:?}", start.elapsed());
}

fn process_chunk(chunk: &[u8], records: &mut HashMap<String, Temperature>, count: &mut usize) {
    let chunk = std::str::from_utf8(chunk).unwrap();
    for line in chunk.lines() {
        *count += 1;
        let (name, temp) = line.split_once(';').unwrap();
        let temp = temp.parse::<f32>().unwrap();
        if let Some(t) = records.get_mut(name) {
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
            records.insert(name.to_string(), Temperature::new(temp, temp, temp));
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // fn test_soln2a() {
    //     soln2a();
    // }
    #[test]
    fn test_soln2() {
        soln2();
    }
}
