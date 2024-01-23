# Benchmarks

## Table of Contents

- [Overview](#overview)
- [Benchmark Results](#benchmark-results)
    - [Populate](#populate)
    - [Access](#access)
    - [Iterate](#iterate)
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
| **`100`**      | `2.40 us` (✅ **1.00x**)   | `2.31 us` (✅ **1.04x faster**)   | `547.37 ns` (🚀 **4.38x faster**) | `483.12 ns` (🚀 **4.96x faster**)  |
| **`10000`**    | `181.80 us` (✅ **1.00x**) | `182.78 us` (✅ **1.01x slower**) | `31.15 us` (🚀 **5.84x faster**)  | `28.25 us` (🚀 **6.43x faster**)   |
| **`10000000`** | `417.15 ms` (✅ **1.00x**) | `424.94 ms` (✅ **1.02x slower**) | `117.54 ms` (🚀 **3.55x faster**) | `115.57 ms` (🚀 **3.61x faster**)  |

### Access

This benchmarks accessing each list structure *without bounds checks*.

The structures provided by this crate are expected to perform slightly worse
than the `Vec` equivalents as they require two accesses (one for metadata,
and one for the actual data), while the `Vec` equivalents only need one.

|                | `Vec<String>`           | `Vec<Vec<u8>>`                 | `CompactStrings`               | `CompactBytestrings`            |
|:---------------|:------------------------|:-------------------------------|:-------------------------------|:------------------------------- |
| **`100`**      | `1.11 ns` (✅ **1.00x**) | `1.10 ns` (✅ **1.01x faster**) | `1.20 ns` (✅ **1.08x slower**) | `1.20 ns` (✅ **1.08x slower**)  |
| **`10000`**    | `1.09 ns` (✅ **1.00x**) | `1.10 ns` (✅ **1.00x slower**) | `1.20 ns` (✅ **1.09x slower**) | `1.20 ns` (✅ **1.09x slower**)  |
| **`10000000`** | `1.10 ns` (✅ **1.00x**) | `1.10 ns` (✅ **1.00x faster**) | `1.20 ns` (✅ **1.09x slower**) | `1.20 ns` (✅ **1.09x slower**)  |

### Iterate

This benchmarks iterating over each list structure.

The structures provided by this crate are expected to perform worse
than the `Vec` equivalents as they require two accesses (one for metadata,
and one for the actual data), while the `Vec` equivalents only need one.

|                | `Vec<String>`            | `Vec<Vec<u8>>`                  | `CompactStrings`                | `CompactBytestrings`             |
|:---------------|:-------------------------|:--------------------------------|:--------------------------------|:-------------------------------- |
| **`100`**      | `24.44 ns` (✅ **1.00x**) | `25.43 ns` (✅ **1.04x slower**) | `40.88 ns` (❌ *1.67x slower*)   | `40.88 ns` (❌ *1.67x slower*)    |
| **`10000`**    | `2.35 us` (✅ **1.00x**)  | `2.38 us` (✅ **1.01x slower**)  | `4.62 us` (❌ *1.96x slower*)    | `4.61 us` (❌ *1.96x slower*)     |
| **`10000000`** | `2.35 ms` (✅ **1.00x**)  | `2.38 ms` (✅ **1.01x slower**)  | `7.78 ms` (❌ *3.30x slower*)    | `7.78 ms` (❌ *3.31x slower*)     |

### Remove First Element

This benchmarks removing the first element of each list structure.

The structures provided by this crate are expected to perform *significantly* worse
on this benchmark as all (byte)strings stored after the removed element need to
be shifted, causing a large reallocation.

The API provides the `ignore` method which only removes the metadata to combat this
at the cost of stale memory.

This benchmark is intentionally biased against the structures provided by this crate
in order to demonstrate pathological behaviour.

|                | `Vec<String>`            | `Vec<Vec<u8>>`                  | `in CompactStrings`              | `CompactBytestrings`              |
|:---------------|:-------------------------|:--------------------------------|:---------------------------------|:--------------------------------- |
| **`100`**      | `41.63 ns` (✅ **1.00x**) | `41.91 ns` (✅ **1.01x slower**) | `112.45 ns` (❌ *2.70x slower*)   | `113.17 ns` (❌ *2.72x slower*)    |
| **`10000`**    | `3.13 us` (✅ **1.00x**)  | `3.12 us` (✅ **1.00x faster**)  | `12.54 us` (❌ *4.00x slower*)    | `12.34 us` (❌ *3.94x slower*)     |
| **`10000000`** | `16.33 ms` (✅ **1.00x**) | `23.05 ms` (❌ *1.41x slower*)   | `69.85 ms` (❌ *4.28x slower*)    | `69.92 ms` (❌ *4.28x slower*)     |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

