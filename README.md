# compact_strings

A cache-friendly but limited representation of a list of strings.

Not affiliated with [compact_str](https://github.com/ParkMyCar/compact_str), 
this crate does not perform the same optimizations.

## About

Strings are stored contiguously in a vector of bytes, with their lengths and 
starting indices being stored separately, rather than as a String struct 
holding a pointer to the data in `Vec<String>`.

This means that there are fewer pointers that need to be followed than in a
`Vec<String>` which should lead to better cache locality, though this has yet
to be tested.

Limitations include being unable to mutate strings stored in the vector.

This could be worked around with a limited API for mutation, but the cost of
moving the rest of the bytes will be much higher than with a `Vec<String>`.

## Benchmarks

Some benchmarks of operations expected to perform vastly differently from a
`Vec<String>` have been benchmarked, you can view them in [here](benches/RESULTS.md)
