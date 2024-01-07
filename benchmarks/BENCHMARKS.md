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

This benchmark is slightly biased towards the structures provided by this crate.

|                | `Vec<String>`             | `Vec<Vec<u8>>`                   | `CompactStrings`                 | `CompactBytestrings`              |
|:---------------|:--------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`100`**      | `2.28 us` (✅ **1.00x**)   | `2.30 us` (✅ **1.01x slower**)   | `561.31 ns` (🚀 **4.06x faster**) | `496.81 ns` (🚀 **4.58x faster**)  |
| **`10000`**    | `177.75 us` (✅ **1.00x**) | `183.15 us` (✅ **1.03x slower**) | `30.85 us` (🚀 **5.76x faster**)  | `27.84 us` (🚀 **6.38x faster**)   |
| **`10000000`** | `429.24 ms` (✅ **1.00x**) | `439.21 ms` (✅ **1.02x slower**) | `118.54 ms` (🚀 **3.62x faster**) | `115.66 ms` (🚀 **3.71x faster**)  |

### Access

This benchmarks accessing each list structure *without bounds checks*.

The structures provided by this crate are expected to perform slightly worse
than the `Vec` equivalents as they require two accesses (one for metadata,
and one for the actual data), while the `Vec` equivalents only need one.

|                | `Vec<String>`           | `Vec<Vec<u8>>`                 | `CompactStrings`               | `CompactBytestrings`            |
|:---------------|:------------------------|:-------------------------------|:-------------------------------|:------------------------------- |
| **`100`**      | `1.10 ns` (✅ **1.00x**) | `1.11 ns` (✅ **1.00x slower**) | `1.19 ns` (✅ **1.08x slower**) | `1.19 ns` (✅ **1.08x slower**)  |
| **`10000`**    | `1.10 ns` (✅ **1.00x**) | `1.11 ns` (✅ **1.01x slower**) | `1.19 ns` (✅ **1.09x slower**) | `1.19 ns` (✅ **1.08x slower**)  |
| **`10000000`** | `1.11 ns` (✅ **1.00x**) | `1.11 ns` (✅ **1.00x faster**) | `1.19 ns` (✅ **1.07x slower**) | `1.18 ns` (✅ **1.07x slower**)  |

### Remove First Element

This benchmarks removing the first element of each list structure, each populated with 10M elements.

The structures provided by this crate are expected to perform *significantly* worse
on this benchmark as all (byte)strings stored after the removed element need to
be shifted, causing a large reallocation.

The API provides the `ignore` and `swap_ignore` methods to combat this
at the cost of stale memory.

This benchmark is intentionally biased against the structures provided by this crate
in order to demonstrate pathological behaviour.

|                | `Vec<String>`            | `Vec<Vec<u8>>`                  | `in CompactStrings`              | `CompactBytestrings`              |
|:---------------|:-------------------------|:--------------------------------|:---------------------------------|:--------------------------------- |
| **`100`**      | `42.16 ns` (✅ **1.00x**) | `42.70 ns` (✅ **1.01x slower**) | `114.92 ns` (❌ *2.73x slower*)   | `114.83 ns` (❌ *2.72x slower*)    |
| **`10000`**    | `3.12 us` (✅ **1.00x**)  | `3.16 us` (✅ **1.01x slower**)  | `12.10 us` (❌ *3.88x slower*)    | `12.11 us` (❌ *3.88x slower*)     |
| **`10000000`** | `16.72 ms` (✅ **1.00x**) | `20.55 ms` (❌ *1.23x slower*)   | `71.42 ms` (❌ *4.27x slower*)    | `70.50 ms` (❌ *4.22x slower*)     |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

