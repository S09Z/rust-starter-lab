use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_send_message(c: &mut Criterion) {
    c.bench_function("send message", |b| b.iter(|| {
        // Call the send_message function
    }));
}

criterion_group!(benches, benchmark_send_message);
criterion_main!(benches);
