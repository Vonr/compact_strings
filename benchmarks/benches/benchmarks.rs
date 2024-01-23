use compact_strings::{CompactBytestrings, CompactStrings};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn populate_str_vec(size: usize) -> Vec<String> {
    let mut vec = Vec::with_capacity(size);

    for _ in 0..size {
        vec.push("lorem ipsum dolor sit amet consectetur adipisci".to_string());
    }

    vec
}

fn populate_byte_vec(size: usize) -> Vec<Vec<u8>> {
    let mut vec = Vec::with_capacity(size);

    for _ in 0..size {
        vec.push(b"lorem ipsum dolor sit amet consectetur adipisci".to_vec());
    }

    vec
}

fn populate_compact_strs(size: usize) -> CompactStrings {
    let mut cmpstrs = CompactStrings::with_capacity(0, size);

    for _ in 0..size {
        cmpstrs.push(black_box("lorem ipsum dolor sit amet consectetur adipisci"));
    }

    cmpstrs
}

fn populate_compact_bytes(size: usize) -> CompactBytestrings {
    let mut cmpbytes = CompactBytestrings::with_capacity(0, size);

    for _ in 0..size {
        cmpbytes.push(black_box(
            b"lorem ipsum dolor sit amet consectetur adipisci",
        ));
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

unsafe fn access_compact_bytes(cmpbytes: &CompactBytestrings, index: usize) -> &[u8] {
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

fn remove_compact_bytes(cmpbytes: &mut CompactBytestrings, index: usize) {
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

    for &size in SIZES {
        bench!(
            format!("Populate/Vec<String>/{size}"),
            populate_str_vec(black_box(size))
        );
        bench!(
            format!("Populate/Vec<Vec<u8>>/{size}"),
            populate_byte_vec(black_box(size))
        );
        bench!(
            format!("Populate/CompactStrings/{size}"),
            populate_compact_strs(black_box(size))
        );
        bench!(
            format!("Populate/CompactBytestrings/{size}"),
            populate_compact_bytes(black_box(size))
        );
    }

    for &size in SIZES {
        let last = size - 1;

        let mut svec = populate_str_vec(size);
        bench!(format!("Access/Vec<String>/{size}"), unsafe {
            access_str_vec(black_box(&svec), black_box(last))
        });
        bench!(format!("Iterate/Vec<String>/{size}"), {
            iter(black_box(&svec))
        });
        bench!(format!("Remove First Element/Vec<String>/{size}"), {
            let vec = &mut svec;
            remove_str_vec(black_box(vec), black_box(0));
            vec.push("lorem ipsum dolor sit amet consectetur adipisci".to_string());
        });
        drop(svec);

        let mut bvec = populate_byte_vec(size);
        bench!(format!("Access/Vec<Vec<u8>>/{size}"), unsafe {
            access_byte_vec(black_box(&bvec), black_box(last))
        });
        bench!(format!("Iterate/Vec<Vec<u8>>/{size}"), {
            iter(black_box(&bvec))
        });
        bench!(format!("Remove First Element/Vec<Vec<u8>>/{size}"), {
            let vec = &mut bvec;
            remove_byte_vec(black_box(vec), black_box(0));
            vec.push(b"lorem ipsum dolor sit amet consectetur adipisci".to_vec());
        });
        drop(bvec);

        let mut scmp = populate_compact_strs(size);
        bench!(format!("Access/CompactStrings/{size}"), unsafe {
            access_compact_strs(black_box(&scmp), black_box(last))
        });
        bench!(format!("Iterate/CompactStrings/{size}"), {
            iter(black_box(&scmp))
        });
        bench!(format!("Remove First Element/in CompactStrings/{size}"), {
            let cmpstrs = &mut scmp;
            remove_compact_strs(black_box(cmpstrs), black_box(0));
            cmpstrs.push("lorem ipsum dolor sit amet consectetur adipisci");
        });
        drop(scmp);

        let mut bcmp = populate_compact_bytes(size);
        bench!(format!("Access/CompactBytestrings/{size}"), unsafe {
            access_compact_bytes(black_box(&bcmp), black_box(last))
        });
        bench!(format!("Iterate/CompactBytestrings/{size}"), {
            iter(black_box(&bcmp))
        });
        bench!(format!("Remove First Element/CompactBytestrings/{size}"), {
            let cmpbytes = &mut bcmp;
            remove_compact_bytes(black_box(cmpbytes), black_box(0));
            cmpbytes.push(b"lorem ipsum dolor sit amet consectetur adipisci");
        });
        drop(bcmp);
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark
}

criterion_main!(benches);
