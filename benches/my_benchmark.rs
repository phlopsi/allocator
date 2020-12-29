use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("v1_17", |b| {
        let a = allocator::v1::Allocator::<i64>::new(17);

        b.iter(|| {
            (
                a.box_it(0),
                a.box_it(1),
                a.box_it(2),
                a.box_it(3),
                a.box_it(4),
                a.box_it(5),
                a.box_it(6),
                a.box_it(7),
                a.box_it(8),
                a.box_it(9),
                a.box_it(10),
                a.box_it(11),
                a.box_it(12),
                a.box_it(13),
                a.box_it(14),
                a.box_it(15),
                a.box_it(16),
            )
        })
    });

    c.bench_function("v2_17", |b| {
        let a = allocator::v2::Allocator::<i64>::new(17);

        b.iter(|| {
            (
                a.box_it(0),
                a.box_it(1),
                a.box_it(2),
                a.box_it(3),
                a.box_it(4),
                a.box_it(5),
                a.box_it(6),
                a.box_it(7),
                a.box_it(8),
                a.box_it(9),
                a.box_it(10),
                a.box_it(11),
                a.box_it(12),
                a.box_it(13),
                a.box_it(14),
                a.box_it(15),
                a.box_it(16),
            )
        })
    });

    c.bench_function("v3_17", |b| {
        let a = allocator::v3::Allocator::<i64>::new(17);

        b.iter(|| {
            (
                a.box_it(0),
                a.box_it(1),
                a.box_it(2),
                a.box_it(3),
                a.box_it(4),
                a.box_it(5),
                a.box_it(6),
                a.box_it(7),
                a.box_it(8),
                a.box_it(9),
                a.box_it(10),
                a.box_it(11),
                a.box_it(12),
                a.box_it(13),
                a.box_it(14),
                a.box_it(15),
                a.box_it(16),
            )
        })
    });

    c.bench_function("v4_17", |b| {
        let a = allocator::v4::Allocator::<i64>::new(17);

        b.iter(|| {
            (
                a.box_it(0),
                a.box_it(1),
                a.box_it(2),
                a.box_it(3),
                a.box_it(4),
                a.box_it(5),
                a.box_it(6),
                a.box_it(7),
                a.box_it(8),
                a.box_it(9),
                a.box_it(10),
                a.box_it(11),
                a.box_it(12),
                a.box_it(13),
                a.box_it(14),
                a.box_it(15),
                a.box_it(16),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
