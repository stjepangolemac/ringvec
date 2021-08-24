use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ringvec::RingVec;

pub fn ringvec_benchmark(c: &mut Criterion) {
    let mut v = RingVec::new(32);

    c.bench_function("push", |b| {
        b.iter(|| {
            v.push(black_box(20));
        })
    });

    c.bench_function("push_force", |b| {
        b.iter(|| {
            v.push_force(black_box(20));
        })
    });

    dbg!(&v);

    c.bench_function("peek_oldest", |b| {
        b.iter(|| {
            v.peek_oldest();
        })
    });

    c.bench_function("peek_newest", |b| {
        b.iter(|| {
            v.peek_newest();
        })
    });
}

criterion_group!(benches, ringvec_benchmark);
criterion_main!(benches);
