use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use crossbeam_utils as cu;
use std::sync::atomic::Ordering::Acquire;
use std::sync::atomic::Ordering::Release;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("box", |b| {
        let repeat = std::sync::atomic::AtomicBool::new(true);

        cu::thread::scope(|s| {
            s.spawn(|_| {
                while repeat.load(Acquire) {
                    std::mem::drop(Box::new(0));
                }
            });

            s.spawn(|_| {
                while repeat.load(Acquire) {
                    std::mem::drop(Box::new(1));
                }
            });

            s.spawn(|_| {
                while repeat.load(Acquire) {
                    std::mem::drop(Box::new(2));
                }
            });

            b.iter(|| {
                (
                    Box::new(3),
                    Box::new(4),
                    Box::new(5),
                    Box::new(6),
                    Box::new(7),
                    Box::new(8),
                    Box::new(9),
                    Box::new(10),
                    Box::new(11),
                    Box::new(12),
                    Box::new(13),
                    Box::new(14),
                    Box::new(15),
                    Box::new(16),
                )
            });

            repeat.store(false, Release);
        })
        .unwrap();
    });

    let group = c.benchmark_group("safe::basic");

    group.bench_function("safe::basic::std{17}", |b| {
        let a = allocator::safe::basic::std::Allocator::<i64>::new(17);
        let repeat = std::sync::atomic::AtomicBool::new(true);

        cu::thread::scope(|s| {
            s.spawn(|_| {
                while repeat.load(Acquire) {
                    std::mem::drop(a.box_it(0));
                }
            });

            s.spawn(|_| {
                while repeat.load(Acquire) {
                    std::mem::drop(a.box_it(1));
                }
            });

            s.spawn(|_| {
                while repeat.load(Acquire) {
                    std::mem::drop(a.box_it(2));
                }
            });

            b.iter(|| {
                (
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
            });

            repeat.store(false, Release);
        })
        .unwrap();
    });

    group.bench_function("safe::basic::parking_lot{17}", |b| {
        let a =
            allocator::safe::basic::parking_lot::Allocator::<i64>::new(
                17,
            );
        let repeat = std::sync::atomic::AtomicBool::new(true);

        cu::thread::scope(|s| {
            s.spawn(|_| {
                while repeat.load(Acquire) {
                    std::mem::drop(a.box_it(0));
                }
            });

            s.spawn(|_| {
                while repeat.load(Acquire) {
                    std::mem::drop(a.box_it(1));
                }
            });

            s.spawn(|_| {
                while repeat.load(Acquire) {
                    std::mem::drop(a.box_it(2));
                }
            });

            b.iter(|| {
                (
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
            });

            repeat.store(false, Release);
        })
        .unwrap();
    });

    group.bench_function("safe::basic::simple_mutex{17}", |b| {
        let a =
            allocator::safe::basic::simple_mutex::Allocator::<i64>::new(
                17,
            );
        let repeat = std::sync::atomic::AtomicBool::new(true);

        cu::thread::scope(|s| {
            s.spawn(|_| {
                while repeat.load(Acquire) {
                    std::mem::drop(a.box_it(0));
                }
            });

            s.spawn(|_| {
                while repeat.load(Acquire) {
                    std::mem::drop(a.box_it(1));
                }
            });

            s.spawn(|_| {
                while repeat.load(Acquire) {
                    std::mem::drop(a.box_it(2));
                }
            });

            b.iter(|| {
                (
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
            });

            repeat.store(false, Release);
        })
        .unwrap();
    });

    group.bench_function("safe::basic::antidote{17}", |b| {
        let a =
            allocator::safe::basic::antidote::Allocator::<i64>::new(17);
        let repeat = std::sync::atomic::AtomicBool::new(true);

        cu::thread::scope(|s| {
            s.spawn(|_| {
                while repeat.load(Acquire) {
                    std::mem::drop(a.box_it(0));
                }
            });

            s.spawn(|_| {
                while repeat.load(Acquire) {
                    std::mem::drop(a.box_it(1));
                }
            });

            s.spawn(|_| {
                while repeat.load(Acquire) {
                    std::mem::drop(a.box_it(2));
                }
            });

            b.iter(|| {
                (
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
            });

            repeat.store(false, Release);
        })
        .unwrap();
    });

    group.finish();

    c.bench_function("safe::advanced::v1{17}", |b| {
        let a =
            allocator::safe::advanced::v1::Allocator::<i64>::new(17);
        let repeat = std::sync::atomic::AtomicBool::new(true);

        cu::thread::scope(|s| {
            s.spawn(|_| {
                while repeat.load(Acquire) {
                    std::mem::drop(a.box_it(0));
                }
            });

            s.spawn(|_| {
                while repeat.load(Acquire) {
                    std::mem::drop(a.box_it(1));
                }
            });

            s.spawn(|_| {
                while repeat.load(Acquire) {
                    std::mem::drop(a.box_it(2));
                }
            });

            b.iter(|| {
                (
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
            });

            repeat.store(false, Release);
        })
        .unwrap();
    });

    c.bench_function("safe::advanced::v2{17}", |b| {
        let a =
            allocator::safe::advanced::v2::Allocator::<i64>::new(17);
        let repeat = std::sync::atomic::AtomicBool::new(true);

        cu::thread::scope(|s| {
            s.spawn(|_| {
                while repeat.load(Acquire) {
                    std::mem::drop(a.box_it(0));
                }
            });

            s.spawn(|_| {
                while repeat.load(Acquire) {
                    std::mem::drop(a.box_it(1));
                }
            });

            s.spawn(|_| {
                while repeat.load(Acquire) {
                    std::mem::drop(a.box_it(2));
                }
            });

            b.iter(|| {
                (
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
            });

            repeat.store(false, Release);
        })
        .unwrap();
    });

    c.bench_function("safe::advanced::v3{17}", |b| {
        let a =
            allocator::safe::advanced::v3::Allocator::<i64>::new(17);
        let repeat = std::sync::atomic::AtomicBool::new(true);

        cu::thread::scope(|s| {
            s.spawn(|_| {
                while repeat.load(Acquire) {
                    std::mem::drop(a.box_it(0));
                }
            });

            s.spawn(|_| {
                while repeat.load(Acquire) {
                    std::mem::drop(a.box_it(1));
                }
            });

            s.spawn(|_| {
                while repeat.load(Acquire) {
                    std::mem::drop(a.box_it(2));
                }
            });

            b.iter(|| {
                (
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
            });

            repeat.store(false, Release);
        })
        .unwrap();
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
