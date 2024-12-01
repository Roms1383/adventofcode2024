use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use day_01::{parse, similarities_v1, similarities_v2, SAMPLE};

fn bench_similarities(c: &mut Criterion) {
    let mut group = c.benchmark_group("Similarities");
    for pair in [parse(SAMPLE), parse(include_str!("../part-II.txt"))].iter() {
        group.bench_with_input(
            BenchmarkId::new("Normal", format!("{pair:?}")),
            pair,
            |b, pair| b.iter(|| similarities_v1(black_box(&pair.0), black_box(&pair.1))),
        );
        group.bench_with_input(
            BenchmarkId::new("Seen", format!("{pair:?}")),
            pair,
            |b, pair| b.iter(|| similarities_v2(black_box(&pair.0), black_box(&pair.1))),
        );
    }
    group.finish();
}

criterion_group!(benches, bench_similarities);
criterion_main!(benches);
