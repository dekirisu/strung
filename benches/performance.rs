use criterion::{black_box, criterion_group, criterion_main, Criterion};
use strung::prelude::*;

#[derive(Strung)]   // easy derive
struct Test {
    num: u32,
    name: &'static str,
}

fn criterion_benchmark(c: &mut Criterion) {
    strung::set_static("{","}");
    let mut group = c.benchmark_group("Performance");
    let string = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, 
    sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, 
    sed diam {name} voluptua. At vero eos et accusam {num} et justo duo dolores et ea rebum. 
    Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. 
    Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor 
    invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. {num} At vero eos et accusam 
    et justo duo {num} dolores et ea rebum. Stet clita kasd gubergren, no sea takimata {name} sanctus est 
    Lorem {name} ipsum dolor sit amet.".to_owned();
    let named = Test {num: 1, name: "st"};
    group.bench_function("default/prefabs/per-struct", |b| b.iter(|| named.strung(black_box(&string))));
    group.bench_function("global", |b| b.iter(|| named.strung_static(black_box(&string))));
    group.bench_function("dynamic", |b| b.iter(|| named.strung_dynamic("{","}",black_box(&string))));
    group.bench_function("generic", |b| b.iter(|| named.strung_generic::<'{','}'>(black_box(&string))));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);