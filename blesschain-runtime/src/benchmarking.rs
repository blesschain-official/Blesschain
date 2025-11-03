//! Benchmarking placeholder (required by construct_runtime).

#![allow(dead_code)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking {
    use frame_benchmarking::v2::{benchmarks, BenchmarkError};

    benchmarks! {
        dummy_benchmark {
            let x = 1;
        }: {
            assert_eq!(x, 1);
        }
    }
}
