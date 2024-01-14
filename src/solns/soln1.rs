use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    num::NonZeroUsize,
    sync::{Arc, Mutex, RwLock},
    thread,
};

use memmap::MmapOptions;
use rayon::prelude::*;

use crate::solns::Temperature;

pub fn soln1() {
    let cores = thread::available_parallelism().unwrap_or(NonZeroUsize::new(5).unwrap());
    let file = File::open("measurements.txt").expect("Failed opening file");
    let mmap = unsafe { MmapOptions::new().map(&file).expect("oops") };
    let records = Arc::new(RwLock::new(HashMap::<String, Mutex<Temperature>>::new()));

    let mut chunks = vec![Vec::new(); cores.into()];
    let mut chunk = Vec::new();
    let chunk_size = mmap.len() / cores;
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

    chunks.par_iter_mut().for_each(|c| {
        process_chunk(c.to_vec(), Arc::clone(&records));
    });

    let records = records.read().unwrap();
    let mut keys = records.keys().collect::<Vec<_>>();
    keys.par_sort_unstable();
    keys.par_iter_mut().for_each(|key| {
        let t = records.get(*key).unwrap().lock().unwrap();
        println!("{}={:.1}/{:.1}/{:.1}", key, t.min, t.mean(), t.max);
    });
}

fn process_chunk(chunk: Vec<String>, data: Arc<RwLock<HashMap<String, Mutex<Temperature>>>>) {
    for line in chunk {
        let mut fields = line.split(';');
        let name = fields.next().unwrap().to_string();
        let temp = fields.next().unwrap().parse::<f32>().unwrap();
        let mut records = data.write().unwrap();
        if let Some(record) = records.get_mut(&name) {
            let t = record.get_mut().unwrap();
            if temp < t.min {
                t.min = temp;
            }
            if temp > t.max {
                t.max = temp;
            }
            t.sum += temp;
            t.count += 1;
        } else {
            records.insert(name, Mutex::new(Temperature::new(temp, temp, temp)));
        }
        drop(records)
    }
}
