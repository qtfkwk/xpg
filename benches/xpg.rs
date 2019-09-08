use criterion::Criterion;
use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;

use xpg::xpg;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("xpg", |b| b.iter(|| xpg(black_box(4))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
