# compact_strings

A more compact but limited representation of a list of strings or bytestrings.

This crate is not affiliated with [compact_str](https://github.com/ParkMyCar/compact_str)
and does not perform the same optimizations.

## About

`Vec<u8>`, which also backs `String`, are 3 pointer-widths each.  
They consist of a pointer to the start of the data, a length denoting how many elements
are currently in the `Vec`, and a capacity denoting the number of elements the
allocation pointed to can hold.

The capacity may not be needed when stored in a list structure, especially when
the (byte)strings are immutable. Furthermore, since each (byte)string uses its
own allocation, large lists will create many allocations, which can be quite slow.

This crate instead stores lists of (byte)strings as two vectors:  
1. Metadata - which holds the starting indexes of the (byte)strings
2. Data - which holds the actual bytes of the (byte)strings

This means that we pay an upfront cost of 6 pointer-widths compared to just 3,
but have a slower growing auxiliary memory consumption of `2n + 6` compared to `3n + 3` pointer-widths,
in addition to being able to store all strings in one fast-growing allocation.

Unfortunately, this structure makes mutating (byte)strings stored in the data vector
extremely difficult without shifting the rest of the data around.  
This could be worked around with a limited API for mutation, but the cost of
moving the rest of the bytes will be much higher than with a `Vec<String>`.  

See [benchmarks](benchmarks/BENCHMARKS.md) for more details.

### New in 4.1.0

This crate now has even more compact versions of the above data structures by
getting rid of the length of the (byte)strings present in the above structures' `Metadata`.

This means that we still pay an upfront cost of 6 pointer-widths rather than 3,
but have an even slower growing auxiliary memory consumption of just `n + 6` pointer-widths.

These are not expected to perform significantly differently from their older counterparts 
and are thus not benchmarked.

## Benchmarks

Some benchmarks of operations expected to perform vastly differently from their
`Vec` equivalents have been benchmarked, you can view them [here](benchmarks/BENCHMARKS.md).
