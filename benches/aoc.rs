use aoc_henk::{map_file, PROGS};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    for (name, progs) in PROGS {
        let input = map_file(format!("input/{}.in", name)).unwrap();
        let mut group = c.benchmark_group(*name);
        progs.iter().enumerate().for_each(|(i, prog)| {
            group.bench_function(format!("{}", i), |b| b.iter(|| prog(black_box(&input))));
        });
        group.finish()
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
