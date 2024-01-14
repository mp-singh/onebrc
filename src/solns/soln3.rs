use std::{
    fs::File,
    io::{BufRead, BufReader},
    sync::{mpsc, Arc, Mutex},
    thread,
};

use dashmap::DashMap as HashMap;

use crate::solns::Temperature;

pub fn soln3() {
    // Open the file
    let file = File::open("measurements.txt").expect("Failed opening file");
    let reader = BufReader::new(file);
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));

    // Determine the number of available cores
    let num_cores = 150;

    let handler_sender = std::thread::spawn(move || {
        let mut chunk = Vec::new();
        for line in reader.lines() {
            if chunk.len() == num_cores {
                sender.send(chunk).unwrap();
                chunk = Vec::new();
            } else {
                chunk.push(line.unwrap());
            }
        }
        drop(sender); // Signal that no more data will be sent
    });

    let records = HashMap::<String, Temperature>::new();

    let handler_rec = thread::spawn(move || {
        let receiver = receiver.lock().unwrap();
        for chunk in receiver.iter() {
            process_chunk(chunk, &records);
        }
    });

    handler_sender.join().unwrap();
    handler_rec.join().unwrap();

    // let keys = &mut records
    //     .iter()
    //     .map(|key| key.key().clone())
    //     .collect::<Vec<_>>();
    // keys.par_sort_unstable();
    // for key in keys {
    //     let _t = records.get(&*key).unwrap();
    //     // println!("{}={:.1}/{:.1}/{:.1}", key, t.min, t.mean(), t.max);
    // }
}

fn process_chunk(chunk: Vec<String>, data: &HashMap<String, Temperature>) {
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
