#![allow(clippy::ptr_arg)]

use compact_strings::FixedCompactStrings;
use honggfuzz::fuzz;
use rutenspitz::arbitrary_stateful_operations;

arbitrary_stateful_operations! {
    model = Vec<String>,
    tested = FixedCompactStrings,

    methods {
        equal {
            fn push(&mut self, element: String);
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

trait AsStr {
    fn as_str(&self) -> &str;
}

impl AsStr for str {
    fn as_str(&self) -> &str {
        self
    }
}

impl AsStr for String {
    fn as_str(&self) -> &str {
        String::as_str(self)
    }
}

fn option_as_str(option: Option<&(impl AsStr + ?Sized)>) -> Option<&str> {
    option.map(|s| AsStr::as_str(s))
}

fn collect<'a>(iter: impl Iterator<Item = &'a (impl AsStr + 'a + ?Sized)> + 'a) -> Vec<&'a str> {
    iter.map(|s| s.as_str()).collect()
}

#[allow(clippy::unnecessary_wraps)]
fn fuzz_cycle(data: &[u8]) -> arbitrary::Result<()> {
    use arbitrary::{Arbitrary, Unstructured};

    let mut ring = Unstructured::new(data);

    let mut tested = FixedCompactStrings::new();

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
