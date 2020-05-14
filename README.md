# rust-thread-comparative
A simple benchmarking of different rust thread crates based on Exercisms
"Parallel letter frequency" exercise.

## How to run

To test that each solution is doing what it should, you can run:
```shell
cargo test -- --ignored
```
To run the benchmark:
```shell
cargo bench
```

## Why
After completing the exercise I became curious about the different
crates available for solving the exercise and how the would compare, so
I created all three versions (standard threads, rayon and crossbeam) 
modified the benchmarks to check.

## About the validity of the tests
I'm still learning about thread management in Rust, and part of the
purpose of this repository is to learn about the different options when
working with parallel computation. This means that it's likely that each
of the solutions can be improved. **IF YOU KNOW HOW, PLEASE DO**,
contributions are very much appreciated.

At this point I think there's minor differences between the codes that
I'll try to fix at some point.

