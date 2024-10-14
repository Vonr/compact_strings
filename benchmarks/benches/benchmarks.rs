use compact_strings::{
    CompactBytestrings, CompactStrings, FixedCompactBytestrings, FixedCompactStrings,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn populate_str_vec(size: usize, s: &str) -> Vec<String> {
    let mut vec = Vec::with_capacity(size);

    for _ in 0..size {
        vec.push(s.to_string());
    }

    vec
}

fn populate_byte_vec(size: usize, s: &str) -> Vec<Vec<u8>> {
    let mut vec = Vec::with_capacity(size);

    for _ in 0..size {
        vec.push(s.as_bytes().to_vec());
    }

    vec
}

fn populate_compact_strs(size: usize, s: &str) -> CompactStrings {
    let mut cmpstrs = CompactStrings::with_capacity(0, size);

    for _ in 0..size {
        cmpstrs.push(black_box(s));
    }

    cmpstrs
}

fn populate_fixed_compact_strs(size: usize, s: &str) -> FixedCompactStrings {
    let mut cmpstrs = FixedCompactStrings::with_capacity(0, size);

    for _ in 0..size {
        cmpstrs.push(black_box(s));
    }

    cmpstrs
}

fn populate_compact_bytes(size: usize, s: &str) -> CompactBytestrings {
    let mut cmpbytes = CompactBytestrings::with_capacity(0, size);

    for _ in 0..size {
        cmpbytes.push(black_box(s.as_bytes()));
    }

    cmpbytes
}

fn populate_fixed_compact_bytes(size: usize, s: &str) -> FixedCompactBytestrings {
    let mut cmpbytes = FixedCompactBytestrings::with_capacity(0, size);

    for _ in 0..size {
        cmpbytes.push(black_box(s.as_bytes()));
    }

    cmpbytes
}

#[allow(clippy::ptr_arg)]
unsafe fn access_str_vec(vec: &Vec<String>, index: usize) -> &str {
    vec.get_unchecked(index)
}

#[allow(clippy::ptr_arg)]
unsafe fn access_byte_vec(vec: &Vec<Vec<u8>>, index: usize) -> &[u8] {
    vec.get_unchecked(index)
}

unsafe fn access_compact_strs(cmpstrs: &CompactStrings, index: usize) -> &str {
    cmpstrs.get_unchecked(index)
}

unsafe fn access_fixed_compact_strs(cmpstrs: &FixedCompactStrings, index: usize) -> &str {
    cmpstrs.get_unchecked(index)
}

unsafe fn access_compact_bytes(cmpbytes: &CompactBytestrings, index: usize) -> &[u8] {
    cmpbytes.get_unchecked(index)
}

unsafe fn access_fixed_compact_bytes(cmpbytes: &FixedCompactBytestrings, index: usize) -> &[u8] {
    cmpbytes.get_unchecked(index)
}

fn remove_str_vec(vec: &mut Vec<String>, index: usize) {
    vec.remove(index);
}

fn remove_byte_vec(vec: &mut Vec<Vec<u8>>, index: usize) {
    vec.remove(index);
}

fn remove_compact_strs(cmpstrs: &mut CompactStrings, index: usize) {
    cmpstrs.remove(index);
}

fn remove_fixed_compact_strs(cmpstrs: &mut FixedCompactStrings, index: usize) {
    cmpstrs.remove(index);
}

fn remove_compact_bytes(cmpbytes: &mut CompactBytestrings, index: usize) {
    cmpbytes.remove(index);
}

fn remove_fixed_compact_bytes(cmpbytes: &mut FixedCompactBytestrings, index: usize) {
    cmpbytes.remove(index);
}

fn iter<T, I: IntoIterator<Item = T>>(iterator: I) {
    for e in iterator {
        std::hint::black_box(e);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    macro_rules! bench {
        ($name:expr, $code:expr) => {
            c.bench_function(&*$name, |b| b.iter(|| $code));
        };
    }

    const SIZES: &[usize] = &[100, 10_000, 10_000_000];
    const STRINGS: &[&'static str] = &["hello", "sphinx of black quartz, judge my vow"];

    for s in STRINGS {
        let (name, _) = s.split_once(" ").unwrap_or((s, ""));

        for &size in SIZES {
            bench!(
                format!("Populate/Vec<String>-{name}/{size}"),
                populate_str_vec(black_box(size), s)
            );
            bench!(
                format!("Populate/Vec<Vec<u8>>-{name}/{size}"),
                populate_byte_vec(black_box(size), s)
            );
            bench!(
                format!("Populate/CompactStrings-{name}/{size}"),
                populate_compact_strs(black_box(size), s)
            );
            bench!(
                format!("Populate/FixedCompactStrings-{name}/{size}"),
                populate_fixed_compact_strs(black_box(size), s)
            );
            bench!(
                format!("Populate/CompactBytestrings-{name}/{size}"),
                populate_compact_bytes(black_box(size), s)
            );
            bench!(
                format!("Populate/FixedCompactBytestrings-{name}/{size}"),
                populate_fixed_compact_bytes(black_box(size), s)
            );
        }
    }

    for s in STRINGS {
        let (name, _) = s.split_once(" ").unwrap_or((s, ""));

        for &size in SIZES {
            let last = size - 1;

            let mut svec = populate_str_vec(size, s);
            bench!(format!("Access/Vec<String>-{name}/{size}"), unsafe {
                access_str_vec(black_box(&svec), black_box(last))
            });
            bench!(format!("Iterate/Vec<String>-{name}/{size}"), {
                iter(black_box(&svec))
            });
            bench!(format!("Remove First Element/Vec<String>-{name}/{size}"), {
                let vec = &mut svec;
                remove_str_vec(black_box(vec), black_box(0));
                vec.push("lorem ipsum dolor sit amet consectetur adipisci".to_string());
            });
            drop(svec);

            let mut bvec = populate_byte_vec(size, s);
            bench!(format!("Access/Vec<Vec<u8>>-{name}/{size}"), unsafe {
                access_byte_vec(black_box(&bvec), black_box(last))
            });
            bench!(format!("Iterate/Vec<Vec<u8>>-{name}/{size}"), {
                iter(black_box(&bvec))
            });
            bench!(
                format!("Remove First Element/Vec<Vec<u8>>-{name}/{size}"),
                {
                    let vec = &mut bvec;
                    remove_byte_vec(black_box(vec), black_box(0));
                    vec.push(b"lorem ipsum dolor sit amet consectetur adipisci".to_vec());
                }
            );
            drop(bvec);

            let mut scmp = populate_compact_strs(size, s);
            bench!(format!("Access/CompactStrings-{name}/{size}"), unsafe {
                access_compact_strs(black_box(&scmp), black_box(last))
            });
            bench!(format!("Iterate/CompactStrings-{name}/{size}"), {
                iter(black_box(&scmp))
            });
            bench!(
                format!("Remove First Element/in CompactStrings-{name}/{size}"),
                {
                    let cmpstrs = &mut scmp;
                    remove_compact_strs(black_box(cmpstrs), black_box(0));
                    cmpstrs.push("lorem ipsum dolor sit amet consectetur adipisci");
                }
            );
            drop(scmp);

            let mut fscmp = populate_fixed_compact_strs(size, s);
            bench!(
                format!("Access/FixedCompactStrings-{name}/{size}"),
                unsafe { access_fixed_compact_strs(black_box(&fscmp), black_box(last)) }
            );
            bench!(format!("Iterate/FixedCompactStrings-{name}/{size}"), {
                iter(black_box(&fscmp))
            });
            bench!(
                format!("Remove First Element/in FixedCompactStrings-{name}/{size}"),
                {
                    let cmpstrs = &mut fscmp;
                    remove_fixed_compact_strs(black_box(cmpstrs), black_box(0));
                    cmpstrs.push("lorem ipsum dolor sit amet consectetur adipisci");
                }
            );
            drop(fscmp);

            let mut bcmp = populate_compact_bytes(size, s);
            bench!(format!("Access/CompactBytestrings-{name}/{size}"), unsafe {
                access_compact_bytes(black_box(&bcmp), black_box(last))
            });
            bench!(format!("Iterate/CompactBytestrings-{name}/{size}"), {
                iter(black_box(&bcmp))
            });
            bench!(
                format!("Remove First Element/CompactBytestrings-{name}/{size}"),
                {
                    let cmpbytes = &mut bcmp;
                    remove_compact_bytes(black_box(cmpbytes), black_box(0));
                    cmpbytes.push(b"lorem ipsum dolor sit amet consectetur adipisci");
                }
            );
            drop(bcmp);

            let mut fbcmp = populate_fixed_compact_bytes(size, s);
            bench!(
                format!("Access/FixedCompactBytestrings-{name}/{size}"),
                unsafe { access_fixed_compact_bytes(black_box(&fbcmp), black_box(last)) }
            );
            bench!(format!("Iterate/FixedCompactBytestrings-{name}/{size}"), {
                iter(black_box(&fbcmp))
            });
            bench!(
                format!("Remove First Element/FixedCompactBytestrings-{name}/{size}"),
                {
                    let cmpbytes = &mut fbcmp;
                    remove_fixed_compact_bytes(black_box(cmpbytes), black_box(0));
                    cmpbytes.push(b"lorem ipsum dolor sit amet consectetur adipisci");
                }
            );
            drop(fbcmp);
        }
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark
}

criterion_main!(benches);
