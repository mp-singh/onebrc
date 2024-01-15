use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use dashmap::DashMap;
use rayon::prelude::*;

use crate::solns::Temperature;

pub struct ChunkingIterator<T> {
    iter: T,
    chunk_size: usize,
}

impl<T> ChunkingIterator<T> {
    pub fn new(iter: T, chunk_size: usize) -> Self {
        Self { iter, chunk_size }
    }
}

impl<T> Iterator for ChunkingIterator<T>
where
    T: Iterator,
{
    type Item = Vec<T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chunk = Vec::with_capacity(self.chunk_size);
        for _ in 0..self.chunk_size {
            if let Some(item) = self.iter.next() {
                chunk.push(item);
            } else {
                break;
            }
        }
        if chunk.is_empty() {
            None
        } else {
            Some(chunk)
        }
    }
}

pub fn soln2() {
    let chunk_size = 100_000;
    let file = File::open("measurements.txt").expect("Failed opening file");
    // let lines = ChunkingIterator::new(BufReader::with_capacity(131072, file).lines(), chunk_size);
    let lines = ChunkingIterator::new(BufReader::new(file).lines(), chunk_size);

    let records = DashMap::<String, Temperature>::new();
    lines.into_iter().par_bridge().for_each(|lines| {
        for line in lines {
            let line = line.unwrap();
            let (name, temp) = line.split_once(';').unwrap();
            let temp = temp.parse::<f32>().unwrap();
            if let Some(mut record) = records.get_mut(name) {
                let t = record.value_mut();
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

    let read = records.into_read_only();
    let mut keys = read.keys().collect::<Vec<_>>();
    keys.par_sort_unstable();
    for key in keys {
        let t = read.get(key).unwrap();
        println!("{}={:.1}/{:.1}/{:.1}", key, t.min, t.mean(), t.max);
    }
}
