use bgpkit_parser::BgpkitParser;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_input() -> String {
    std::env::var("BGP_TEST_FILE")
        .unwrap_or_else(|_| "https://spaces.bgpkit.org/parser/update-example".to_string())
}

fn bench_rust_native_iteration(c: &mut Criterion) {
    let input = bench_input();
    c.bench_function("rust native elem iteration", |b| {
        b.iter(|| {
            let parser = BgpkitParser::new(input.as_str()).expect("create parser");
            let count = parser.into_elem_iter().count();
            black_box(count);
        });
    });
}

fn bench_rust_native_route_iteration(c: &mut Criterion) {
    let input = bench_input();
    c.bench_function("rust native route iteration", |b| {
        b.iter(|| {
            let parser = BgpkitParser::new(input.as_str()).expect("create parser");
            let count = parser.into_route_iter().count();
            black_box(count);
        });
    });
}

criterion_group!(
    benches,
    bench_rust_native_iteration,
    bench_rust_native_route_iteration
);
criterion_main!(benches);
