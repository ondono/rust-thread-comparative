extern crate rayon;

use rayon::prelude::*;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    assert!(
        worker_count > 0,
        "Working with less than 1 worker might prove difficult!"
    );
    if input.is_empty() {
        return HashMap::new();
    }
    // split the text in chunks and give one to each thread
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(worker_count)
        .build()
        .unwrap();

    pool.install(|| {
        input
            .into_par_iter()
            .fold(HashMap::new, |acc, line| {
                line.chars()
                    .filter(|c| c.is_alphabetic())
                    .flat_map(|c| c.to_lowercase())
                    .fold(acc, |mut acc, c| {
                        *acc.entry(c).or_insert(0) += 1usize;
                        acc
                    })
            })
            .reduce(HashMap::new, |mut acc, hm| {
                for (k, v) in hm {
                    *acc.entry(k).or_insert(0) += v;
                }
                acc
            })
    })
}
