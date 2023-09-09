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

fn criterion_benchmark(c: &mut Criterion) {
    macro_rules! bench {
        ($name:literal, $code:expr) => {
            c.bench_function($name, |b| b.iter(|| $code));
        };
    }

    bench!("populate str vec 100", populate_str_vec(black_box(100)));
    bench!(
        "populate bytestring vec 100",
        populate_byte_vec(black_box(100))
    );
    bench!(
        "populate compact strings 100",
        populate_compact_strs(black_box(100))
    );
    bench!(
        "populate compact bytestrings 100",
        populate_compact_bytes(black_box(100))
    );
    bench!("populate str vec 10000", populate_str_vec(black_box(10000)));
    bench!(
        "populate bytestring vec 10000",
        populate_byte_vec(black_box(10000))
    );
    bench!(
        "populate compact strings 10000",
        populate_compact_strs(black_box(10000))
    );
    bench!(
        "populate compact bytestrings 10000",
        populate_compact_bytes(black_box(10000))
    );
    bench!(
        "populate str vec 10000000",
        populate_str_vec(black_box(10000000))
    );
    bench!(
        "populate bytestring vec 10000000",
        populate_byte_vec(black_box(10000000))
    );
    bench!(
        "populate compact strings 10000000",
        populate_compact_strs(black_box(10000000))
    );
    bench!(
        "populate compact bytestrings 10000000",
        populate_compact_bytes(black_box(10000000))
    );

    let size = 10000000;

    {
        let mut svec = populate_str_vec(size);
        bench!("access vec strings", unsafe {
            access_str_vec(black_box(&svec), black_box(9999))
        });
        bench!("remove first vec string", {
            let vec = &mut svec;
            remove_str_vec(black_box(vec), black_box(0));
            vec.push("lorem ipsum dolor sit amet consectetur adipisci".to_string());
        });
    }

    {
        let mut bvec = populate_byte_vec(size);
        bench!("access vec bytestrings", unsafe {
            access_byte_vec(black_box(&bvec), black_box(9999))
        });
        bench!("remove first vec bytestring", {
            let vec = &mut bvec;
            remove_byte_vec(black_box(vec), black_box(0));
            vec.push(b"lorem ipsum dolor sit amet consectetur adipisci".to_vec());
        });
    }

    {
        let mut scmp = populate_compact_strs(size);
        bench!("access compact strings", unsafe {
            access_compact_strs(black_box(&scmp), black_box(9999))
        });
        bench!("remove first compact string", {
            let cmpstrs = &mut scmp;
            remove_compact_strs(black_box(cmpstrs), black_box(0));
            cmpstrs.push("lorem ipsum dolor sit amet consectetur adipisci");
        });
    }

    {
        let mut bcmp = populate_compact_bytes(size);
        bench!("access compact bytestrings", unsafe {
            access_compact_bytes(black_box(&bcmp), black_box(9999))
        });
        bench!("remove first compact bytestring", {
            let cmpbytes = &mut bcmp;
            remove_compact_bytes(black_box(cmpbytes), black_box(0));
            cmpbytes.push(b"lorem ipsum dolor sit amet consectetur adipisci");
        });
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark
}

criterion_main!(benches);
