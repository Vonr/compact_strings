use compact_strings::CompactStrings;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn populate_vec(size: usize) -> Vec<String> {
    let mut vec = Vec::with_capacity(size);

    for _ in 0..size {
        vec.push("lorem ipsum dolor sit amet consectetur adipisci".to_string());
    }

    vec
}

fn populate_compact(size: usize) -> CompactStrings {
    let mut cmpstrs = CompactStrings::with_capacity(0, size);

    for _ in 0..size {
        cmpstrs.push("lorem ipsum dolor sit amet consectetur adipisci");
    }

    cmpstrs
}

#[allow(clippy::ptr_arg)]
unsafe fn access_vec(vec: &Vec<String>, index: usize) -> &str {
    vec.get_unchecked(index)
}

unsafe fn access_compact(cmpstrs: &CompactStrings, index: usize) -> &str {
    cmpstrs.get_unchecked(index)
}

#[allow(clippy::ptr_arg)]
fn remove_vec(vec: &mut Vec<String>, index: usize) {
    vec.remove(index);
}

fn remove_compact(cmpstrs: &mut CompactStrings, index: usize) {
    cmpstrs.remove(index);
}

fn criterion_benchmark(c: &mut Criterion) {
    macro_rules! bench {
        ($name:literal, $code:expr) => {
            c.bench_function($name, |b| b.iter(|| $code));
        };
    }

    bench!("populate vec 20", populate_vec(black_box(20)));
    bench!("populate compact 20", populate_compact(black_box(20)));
    bench!("populate vec 100", populate_vec(black_box(100)));
    bench!("populate compact 100", populate_compact(black_box(100)));
    bench!("populate vec 1000", populate_vec(black_box(1000)));
    bench!("populate compact 1000", populate_compact(black_box(1000)));
    bench!("populate vec 10000", populate_vec(black_box(10000)));
    bench!("populate compact 10000", populate_compact(black_box(10000)));
    bench!("populate vec 100000", populate_vec(black_box(100000)));
    bench!(
        "populate compact 100000",
        populate_compact(black_box(100000))
    );
    bench!("populate vec 1000000", populate_vec(black_box(1000000)));
    bench!(
        "populate compact 1000000",
        populate_compact(black_box(1000000))
    );
    bench!("populate vec 10000000", populate_vec(black_box(10000000)));
    bench!(
        "populate compact 10000000",
        populate_compact(black_box(10000000))
    );

    let mut vec10000 = populate_vec(10000);
    let mut cmp10000 = populate_compact(10000);

    bench!("access vec", unsafe {
        access_vec(black_box(&vec10000), black_box(9999))
    });
    bench!("access compact", unsafe {
        access_compact(black_box(&cmp10000), black_box(9999))
    });

    bench!("remove first vec", {
        let vec = &mut vec10000;
        remove_vec(black_box(vec), black_box(0));
        vec.push("lorem ipsum dolor sit amet consectetur adipisci".to_string());
    });
    bench!("remove last vec", {
        let vec = &mut vec10000;
        remove_vec(black_box(vec), black_box(9999));
        vec.push("lorem ipsum dolor sit amet consectetur adipisci".to_string());
    });

    bench!("remove first compact", {
        let cmpstrs = &mut cmp10000;
        remove_compact(black_box(cmpstrs), black_box(0));
        cmpstrs.push("lorem ipsum dolor sit amet consectetur adipisci");
    });
    bench!("remove last compact", {
        let cmpstrs = &mut cmp10000;
        remove_compact(black_box(cmpstrs), black_box(9999));
        cmpstrs.push("lorem ipsum dolor sit amet consectetur adipisci");
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark
}

criterion_main!(benches);
