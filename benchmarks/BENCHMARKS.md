# Benchmarks

## Table of Contents

- [Overview](#overview)
- [Benchmark Results](#benchmark-results)
    - [Populate](#populate)
    - [Access](#access)
    - [Remove First Element](#remove-first-element)

## Overview

Only a few operations that are thought to have significant differences in
performance from their `Vec` equivalents are benchmarked and shown here.

The benchmark itself uses quite naive testing methods and are by no means
perfect. For example, the removal benchmarks also include the timing to
re-add elements to prevent the benchmark from exhausting the elements.

Furthermore, all the elements are of the same length and may lead to
favourable results for one of the data structures being benchmarked.

Should you have ideas on how to better benchmark this, a pull request
would be greatly appreciated.

Row values, if any, indicate the number of (byte)strings in the list.

In order to reduce variance, I've chosen to populate each list with
"lorem ipsum dolor sit amet consectetur adipisci".

There may be performance implications, as the `Vec` structures likely
prefer larger (byte)strings while the chosen (byte)string is relatively small.

## Benchmark Results

### Populate

This benchmarks the population of each list structure at various sizes.

The structures provided by this crate are expected to perform better than
the `Vec` equivalents as the data vector will grow rapidly in size to
accomodate for large numbers of new (byte)strings while the `Vec` equivalents
need to find and allocate new space for each new (byte)string.

|                | `Vec<String>`             | `Vec<Vec<u8>>`                   | `CompactStrings`                 | `CompactBytestrings`              |
|:---------------|:--------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`100`**      | `2.31 us` (✅ **1.00x**)   | `2.29 us` (✅ **1.01x faster**)   | `643.92 ns` (🚀 **3.59x faster**) | `534.54 ns` (🚀 **4.32x faster**)  |
| **`10000`**    | `174.00 us` (✅ **1.00x**) | `177.69 us` (✅ **1.02x slower**) | `29.99 us` (🚀 **5.80x faster**)  | `27.17 us` (🚀 **6.40x faster**)   |
| **`10000000`** | `417.67 ms` (✅ **1.00x**) | `417.80 ms` (✅ **1.00x slower**) | `112.15 ms` (🚀 **3.72x faster**) | `110.67 ms` (🚀 **3.77x faster**)  |

### Access

This benchmarks accessing each list structure *without bounds checks*.

The structures provided by this crate are expected to perform slightly worse
than the `Vec` equivalents as they require two accesses (one for metadata,
and one for the actual data), while the `Vec` equivalents only need one.

|                | `Vec<String>`           | `Vec<Vec<u8>>`                 | `CompactStrings`               | `CompactBytestrings`            |
|:---------------|:------------------------|:-------------------------------|:-------------------------------|:------------------------------- |
| **`100`**      | `1.09 ns` (✅ **1.00x**) | `1.09 ns` (✅ **1.00x faster**) | `1.16 ns` (✅ **1.06x slower**) | `1.15 ns` (✅ **1.06x slower**)  |
| **`10000`**    | `1.07 ns` (✅ **1.00x**) | `1.08 ns` (✅ **1.01x slower**) | `1.16 ns` (✅ **1.08x slower**) | `1.16 ns` (✅ **1.08x slower**)  |
| **`10000000`** | `1.09 ns` (✅ **1.00x**) | `1.09 ns` (✅ **1.00x faster**) | `1.15 ns` (✅ **1.06x slower**) | `1.16 ns` (✅ **1.07x slower**)  |

### Remove First Element

This benchmarks removing the first element of each list structure.

The structures provided by this crate are expected to perform *significantly* worse
on this benchmark as all (byte)strings stored after the removed element need to
be shifted, causing a large reallocation.

The API provides the `ignore` and `swap_ignore` methods to combat this
at the cost of stale memory.

This benchmark is intentionally biased against the structures provided by this crate
in order to demonstrate pathological behaviour.

|                | `Vec<String>`            | `Vec<Vec<u8>>`                  | `in CompactStrings`              | `CompactBytestrings`              |
|:---------------|:-------------------------|:--------------------------------|:---------------------------------|:--------------------------------- |
| **`100`**      | `41.48 ns` (✅ **1.00x**) | `41.27 ns` (✅ **1.01x faster**) | `109.33 ns` (❌ *2.64x slower*)   | `110.47 ns` (❌ *2.66x slower*)    |
| **`10000`**    | `3.06 us` (✅ **1.00x**)  | `3.06 us` (✅ **1.00x slower**)  | `11.70 us` (❌ *3.82x slower*)    | `11.88 us` (❌ *3.88x slower*)     |
| **`10000000`** | `15.86 ms` (✅ **1.00x**) | `19.62 ms` (❌ *1.24x slower*)   | `58.82 ms` (❌ *3.71x slower*)    | `58.45 ms` (❌ *3.69x slower*)     |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

