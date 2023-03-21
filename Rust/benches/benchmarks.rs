use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate bs;
use bs::binary_search::power::DynamicImplementation;
use bs::binary_search::DynamicBinarySearch;
use bs::helpers::Dynamic;

type DataType = i32;
type IndexType = usize;
const N: usize = 65536;

fn dynamic_power_search_first(c: &mut Criterion) {
    let bs: DynamicImplementation = DynamicImplementation {};
    let vect: Vec<DataType> = Dynamic::filler::<DataType, IndexType>(N);
    let value: DataType = *vect.first().unwrap();

    c.bench_function("dynamic_power_search_first", |b|
                     b.iter(|| {
                         let result: Option<IndexType> = bs.r#impl(black_box(vect.as_slice()), value);
                         assert!(result.is_some());
                     })
    );
}

fn dynamic_power_search_last(c: &mut Criterion) {
    let bs: DynamicImplementation = DynamicImplementation {};
    let vect: Vec<DataType> = Dynamic::filler::<DataType, IndexType>(N);
    let value: DataType = *vect.last().unwrap();

    c.bench_function("dynamic_power_search_last", |b|
                     b.iter(|| {
                         let result: Option<IndexType> = bs.r#impl(black_box(vect.as_slice()), value);
                         assert!(result.is_some());
                     })
    );
}

fn dynamic_power_fail_first(c: &mut Criterion) {
    let bs: DynamicImplementation = DynamicImplementation {};
    let vect: Vec<DataType> = Dynamic::filler::<DataType, IndexType>(N);
    let value: DataType = *vect.first().unwrap() - 1;

    c.bench_function("dynamic_power_fail_first", |b|
                     b.iter(||{
                         let result: Option<IndexType> = bs.r#impl(black_box(vect.as_slice()), value);
                         assert!(result.is_none());
                     })
    );
}

fn dynamic_power_fail_last(c: &mut Criterion) {
    let bs: DynamicImplementation = DynamicImplementation {};
    let vect: Vec<DataType> = Dynamic::filler::<DataType, IndexType>(N);
    let value: DataType = *vect.last().unwrap() + 1;

    c.bench_function("dynamic_power_fail_last", |b|
                     b.iter(||{
                         let result: Option<IndexType> = bs.r#impl(vect.as_slice(), value);
                         assert!(result.is_none());
                     })
    );
}

criterion_group!(
    benches,
    dynamic_power_search_first,
    dynamic_power_search_last,
    dynamic_power_fail_first,
    dynamic_power_fail_last
);
criterion_main!(benches);
