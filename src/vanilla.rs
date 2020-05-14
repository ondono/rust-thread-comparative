use std::collections::HashMap;
use std::thread;

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

    match worker_count {
        // Setting up things for multiple threads and then spawn a single thread
        // sounds kind of unfair to this case. Let's just do it as if we haven't
        // even heard of threads
        1 => {
            // Technically for 1 worker we don't need to create a copy, but it doesn't make much
            // of a difference in bench, and I'd rather have only one counting function
            let data = input
                .iter()
                .copied()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            counter(data)
        }
        _ => {
            let mut threads = Vec::new();
            // split the text in chunks and give one to each thread
            let chunk_size = (input.len()) / worker_count + 1;
            for chunk in input.chunks(chunk_size) {
                let data = chunk
                    .iter()
                    .copied()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();
                let t = thread::spawn(move || counter(data));
                threads.push(t);
            }

            let mut map = HashMap::new();
            // join the hashmaps from each thread
            for thread in threads {
                for (c, count) in thread.join().unwrap() {
                    *map.entry(c).or_insert(0) += count;
                }
            }
            map
        }
    }
}
