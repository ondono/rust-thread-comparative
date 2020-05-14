extern crate crossbeam;

use crossbeam::thread;
use std::collections::HashMap;

// Using the same function for both seems fair
fn counter(input: Vec<String>) -> HashMap<char, usize> {
    let mut map = HashMap::new();

    for string in input {
        for c in string.chars().filter(|c| c.is_alphabetic()) {
            if let Some(x) = c.to_lowercase().next() {
                *map.entry(x).or_insert(0) += 1;
            }
        }
    }
    map
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    assert!(
        worker_count > 0,
        "Working with less than 1 worker might prove difficult!"
    );
    if input.is_empty() {
        return HashMap::new();
    }
    // split the text in chunks and give one to each thread
    let chunk_size = (input.len()) / worker_count + 1;

    let mut map = HashMap::new();
    thread::scope(|s| {
        let threads: Vec<_> = input
            .chunks(chunk_size)
            .map(|chunk| {
                s.spawn(move |_| {
                    counter(
                        chunk
                            .iter()
                            .copied()
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>(),
                    )
                })
            })
            .collect();
        for handle in threads {
            for (k, v) in handle.join().unwrap() {
                *(map.entry(k).or_insert(0)) += v;
            }
        }
    })
    .unwrap();
    // join the hashmaps from each thread

    map
}
