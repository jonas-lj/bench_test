use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_with_input(BenchmarkId::new("fib", 10), &10, |b, input| b.iter(|| fibonacci(black_box(*input))));
    c.bench_with_input(BenchmarkId::new("fib", 20), &20, |b, input| b.iter(|| fibonacci(black_box(*input))));
    c.bench_with_input(BenchmarkId::new("fib", 30), &30, |b, input| b.iter(|| fibonacci(black_box(*input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);