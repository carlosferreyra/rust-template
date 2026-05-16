//! Criterion benchmarks for `{{project-name}}-core`.
//!
//! Gated behind the `bench` feature so `cargo test` stays fast. Run with:
//!
//! ```sh
//! cargo bench -p {{project-name}}-core --features bench
//! ```
#![allow(missing_docs)]

use std::hint::black_box;

use {{crate_name}}_core::greet;
use criterion::{Criterion, criterion_group, criterion_main};

fn bench_greet(c: &mut Criterion) {
    c.bench_function("greet world", |b| {
        b.iter(|| greet(black_box("world")).unwrap());
    });
}

criterion_group!(benches, bench_greet);
criterion_main!(benches);
