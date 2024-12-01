use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use day_01::{parse_v1, parse_v2, SAMPLE};

fn bench_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parse");
    for input in [
        (SAMPLE, "Sample"),
        (include_str!("../part-I.txt"), "Part-I"),
        (include_str!("../part-II.txt"), "Part-II"),
    ]
    .iter()
    {
        group.bench_with_input(BenchmarkId::new("Normal", input.1), input.0, |b, input| {
            b.iter(|| parse_v1(black_box(input)))
        });
        group.bench_with_input(BenchmarkId::new("Reserve", input.1), input.0, |b, input| {
            b.iter(|| parse_v2(black_box(input)))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_parse);
criterion_main!(benches);
