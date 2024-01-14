use std::{
    fs::File,
    io::{BufRead, BufReader},
    sync::Arc,
};

use dashmap::DashMap as HashMap;
use memmap::MmapOptions;
use rayon::prelude::*;

use crate::solns::Temperature;

pub fn soln2() {
    let start = std::time::Instant::now();
    let cores = 160;
    let file = File::open("measurements.txt").expect("Failed opening file");
    let mmap = unsafe { MmapOptions::new().map(&file).expect("oops") };

    let records = HashMap::<String, Temperature>::new();

    let chunk_size = mmap.len() / cores;
    let mut chunks = vec![Vec::new(); cores];
    let mut chunk = Vec::new();

    for line in BufReader::new(mmap.as_ref()).lines() {
        if chunk.len() == chunk_size {
            chunks.push(chunk);
            chunk = Vec::new();
        } else {
            chunk.push(line.unwrap());
        }
    }
    if !chunk.is_empty() {
        chunks.push(chunk);
    }

    let start1 = std::time::Instant::now();
    chunks.par_iter().for_each(|c| {
        let data = Arc::new(&records);
        process_chunk(c.par_iter().map(|x| x.as_str()).collect(), &data);
    });

    let mut keys = records
        .iter()
        .map(|key| key.key().clone())
        .collect::<Vec<_>>();
    keys.par_sort_unstable();
    for key in keys {
        let t = records.get(&key).unwrap();
        println!("{}={:.1}/{:.1}/{:.1}", key, t.min, t.mean(), t.max);
    }
    println!("computation time: {}s", start1.elapsed().as_secs_f32());
    println!("total time: {}s", start.elapsed().as_secs_f32());
}

fn process_chunk(chunk: Vec<&str>, data: &HashMap<String, Temperature>) {
    for line in chunk {
        let mut fields = line.split(';');
        let name = fields.next().unwrap().to_string();
        let temp = fields.next().unwrap().parse::<f32>().unwrap();
        if let Some(mut record) = data.get_mut(&name) {
            let t = record.value_mut();
            if temp < t.min {
                t.min = temp;
            }
            if temp > t.max {
                t.max = temp;
            }
            t.sum += temp;
            t.count += 1;
        } else {
            data.insert(name, Temperature::new(temp, temp, temp));
        }
    }
}
