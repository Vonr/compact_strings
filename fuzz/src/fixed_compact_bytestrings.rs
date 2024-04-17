#![allow(clippy::ptr_arg)]

use compact_strings::FixedCompactBytestrings;
use honggfuzz::fuzz;
use rutenspitz::arbitrary_stateful_operations;

arbitrary_stateful_operations! {
    model = Vec<Vec<u8>>,
    tested = FixedCompactBytestrings,

    methods {
        equal {
            fn push(&mut self, element: Vec<u8>);
            fn len(&self);
            fn is_empty(&self);
            fn clear(&mut self);
        }

        equal_with(option_as_str) {
            fn get(&mut self, index: usize);
        }

        equal_with(collect) {
            fn iter(&self);
        }
    }
}

trait AsBytestring {
    fn as_bstr(&self) -> &[u8];
}

impl AsBytestring for [u8] {
    fn as_bstr(&self) -> &[u8] {
        self
    }
}

impl AsBytestring for Vec<u8> {
    fn as_bstr(&self) -> &[u8] {
        self
    }
}

fn option_as_str(option: Option<&(impl AsBytestring + ?Sized)>) -> Option<&[u8]> {
    option.map(|s| AsBytestring::as_bstr(s))
}

fn collect<'a>(
    iter: impl Iterator<Item = &'a (impl AsBytestring + 'a + ?Sized)> + 'a,
) -> Vec<&'a [u8]> {
    iter.map(|s| s.as_bstr()).collect()
}

#[allow(clippy::unnecessary_wraps)]
fn fuzz_cycle(data: &[u8]) -> arbitrary::Result<()> {
    use arbitrary::{Arbitrary, Unstructured};

    let mut ring = Unstructured::new(data);

    let mut tested = FixedCompactBytestrings::new();

    let mut op_trace = String::new();
    while let Ok(op) = <op::Op as Arbitrary>::arbitrary(&mut ring) {
        op.append_to_trace(&mut op_trace);
        op.execute(&mut tested);
    }

    Ok(())
}

fn main() -> Result<(), ()> {
    better_panic::install();

    loop {
        fuzz!(|data: &[u8]| {
            let _ = fuzz_cycle(data);
        });
    }
}
