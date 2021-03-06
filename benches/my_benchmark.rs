use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use crossbeam_utils as cu;
use std::sync::atomic::Ordering::Acquire;
use std::sync::atomic::Ordering::Release;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("allocator");

    group.bench_function("box{4}", |b| {
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

            b.iter(|| Box::new(3));

            repeat.store(false, Release);
        })
        .unwrap();
    });

    group.bench_function("safe::std{4}", |b| {
        let a = allocator::s::basic::std::Allocator::<i64>::new(4);
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

            b.iter(|| a.box_it(3));

            repeat.store(false, Release);
        })
        .unwrap();
    });

    group.bench_function("safe::parking_lot{4}", |b| {
        let a =
            allocator::s::basic::parking_lot::Allocator::<i64>::new(4);
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

            b.iter(|| a.box_it(3));

            repeat.store(false, Release);
        })
        .unwrap();
    });

    group.bench_function("safe::simple_mutex{4}", |b| {
        let a =
            allocator::s::basic::simple_mutex::Allocator::<i64>::new(4);
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

            b.iter(|| a.box_it(3));

            repeat.store(false, Release);
        })
        .unwrap();
    });

    group.bench_function("safe::antidote{4}", |b| {
        let a = allocator::s::basic::antidote::Allocator::<i64>::new(4);
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

            b.iter(|| a.box_it(3));

            repeat.store(false, Release);
        })
        .unwrap();
    });

    group.bench_function("safe::v1{4}", |b| {
        let a = allocator::s::advanced::v1::Allocator::<i64>::new(4);
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

            b.iter(|| a.box_it(3));

            repeat.store(false, Release);
        })
        .unwrap();
    });

    group.bench_function("safe::v2{4}", |b| {
        let a = allocator::s::advanced::v2::Allocator::<i64>::new(4);
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

            b.iter(|| a.box_it(3));

            repeat.store(false, Release);
        })
        .unwrap();
    });

    group.bench_function("safe::v3{4}", |b| {
        let a = allocator::s::advanced::v3::Allocator::<i64>::new(4);
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

            b.iter(|| a.box_it(3));

            repeat.store(false, Release);
        })
        .unwrap();
    });

    group.bench_function("unsafe::v1{4}", |b| {
        let a = allocator::u::v1::Allocator::<i64>::new(4);
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

            b.iter(|| a.box_it(3));

            repeat.store(false, Release);
        })
        .unwrap();
    });

    group.bench_function("unsafe::v2{4}", |b| {
        let a = allocator::u::v2::Allocator::<i64>::new(4);
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

            b.iter(|| a.box_it(3));

            repeat.store(false, Release);
        })
        .unwrap();
    });

    group.bench_function("unsafe::v4{4}", |b| {
        let a = allocator::u::v4::Allocator::<i64>::new(4);
        let repeat = std::sync::atomic::AtomicBool::new(true);

        cu::thread::scope(|s| {
            s.spawn(|_| {
                let a = a.thread_local();

                while repeat.load(Acquire) {
                    std::mem::drop(a.box_it(0));
                }
            });

            s.spawn(|_| {
                let a = a.thread_local();

                while repeat.load(Acquire) {
                    std::mem::drop(a.box_it(1));
                }
            });

            s.spawn(|_| {
                let a = a.thread_local();

                while repeat.load(Acquire) {
                    std::mem::drop(a.box_it(2));
                }
            });

            let a = a.thread_local();

            b.iter(|| a.box_it(3));

            repeat.store(false, Release);
        })
        .unwrap();
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
