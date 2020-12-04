use aoc_henk::{map_file, PROGS};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    for (name, prog) in PROGS {
        let input = map_file(format!("input/{}.in", name)).unwrap();
        c.bench_function(name, |b| b.iter(|| prog(black_box(&input))));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
