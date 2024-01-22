use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    os::unix::fs::MetadataExt,
};

use rayon::slice::ParallelSliceMut;

use crate::solns::Temperature;

pub fn soln2() {
    let start = std::time::Instant::now();
    let file = File::open("measurements.txt").expect("Failed opening file");
    let mmap = unsafe { memmap::Mmap::map(&file).expect("failed to map file") };
    let file_size = file.metadata().unwrap().size();
    let mut contents = String::with_capacity(file_size as usize);
    let mut reader = BufReader::new(mmap.as_ref());
    reader
        .read_to_string(&mut contents)
        .expect("Failed reading file");

    let num_of_threads = 32;
    let mut chunks = Vec::<String>::with_capacity(num_of_threads as usize);
    let contents = contents.as_bytes().to_vec();

    println!("gathering chunks of size {}", file_size / num_of_threads);
    for i in 0..num_of_threads {
        work(file_size, num_of_threads, i, &contents, &mut chunks);
    }

    // let remaining = contents.len() - chunks.iter().map(|c| c.len()).sum::<usize>();
    // if remaining < (file_size / num_of_threads) as usize {
    //     let chunk = &contents[contents.len() - remaining..];
    //     chunks.push(chunk.iter().map(|c| char::from(*c)).collect::<String>());
    // }
    println!("finished gathering chunks in {:?}", start.elapsed());

    let start1 = std::time::Instant::now();
    let mut records = HashMap::<String, Temperature>::with_capacity(10_000);
    // create an atomic counter
    let count = std::sync::atomic::AtomicUsize::new(0);
    chunks.iter().for_each(|chunks| {
        for line in chunks.lines() {
            count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if line.is_empty() {
                return;
            }
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
    });
    println!("finished processing in {:?}", start1.elapsed());
    let mut keys = records.keys().collect::<Vec<_>>();
    keys.par_sort_unstable();
    for key in keys {
        let t = records.get(key).unwrap();
        println!("{}={:.1}/{:.1}/{:.1}", key, t.min, t.mean(), t.max);
    }
    println!("Time taken: {:?}, \ncount: {:?}", start.elapsed(), count);
}

fn work(file_size: u64, num_of_threads: u64, i: u64, contents: &[u8], chunks: &mut Vec<String>) {
    let mut from = file_size / num_of_threads * i;
    let mut to = file_size / num_of_threads * (i + 1); // exclusive
    if from != 0 {
        while contents[from as usize] != b'\n' {
            from -= 1;
        }
        // exclude the newline
        from += 1;
    }

    while to < file_size && contents[to as usize] != b'\n' {
        to -= 1;
    }

    if to != file_size {
        // include the newline
        to += 1;
    } else {
        to = file_size;
    }

    let chunk = &contents[from as usize..to as usize];
    chunks.push(chunk.iter().map(|c| char::from(*c)).collect::<String>());
}
