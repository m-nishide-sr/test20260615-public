use criterion::{criterion_group, criterion_main, Criterion};
use tokio::runtime::Builder;
use std::hint::black_box;
use hello_cargo::{fibonacci, fibonacci_loop, fibonacci_async};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fibonacci 30", |b| {
        // black_box を使うことで、コンパイラの過剰な最適化（関数のインライン化や計算の事前完了など）を防ぐ
        b.iter(|| fibonacci(black_box(30)))
    });
    c.bench_function("fibonacci_loop 30", |b| {
        // black_box を使うことで、コンパイラの過剰な最適化（関数のインライン化や計算の事前完了など）を防ぐ
        b.iter(|| fibonacci_loop(black_box(30)))
    });
    c.bench_function("fibonacci_async 30 (many threads)", |b| {
        // black_box を使うことで、コンパイラの過剰な最適化（関数のインライン化や計算の事前完了など）を防ぐ
        b.iter(|| tokio::runtime::Runtime::new().unwrap().block_on(fibonacci_async(black_box(30))))
    });

    c.bench_function("fibonacci_async 30 (1 thread)", |b| {
        // スレッド数を1（現在のスレッドのみで動作）にするランタイムを作成
        let rt = Builder::new_current_thread()
            .build()
            .unwrap();

        b.iter(|| {
            rt.block_on(fibonacci_async(black_box(30)))
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);