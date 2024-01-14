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
| **`100`**      | `2.44 us` (✅ **1.00x**)   | `2.36 us` (✅ **1.03x faster**)   | `583.10 ns` (🚀 **4.19x faster**) | `503.16 ns` (🚀 **4.86x faster**)  |
| **`10000`**    | `184.57 us` (✅ **1.00x**) | `187.08 us` (✅ **1.01x slower**) | `31.74 us` (🚀 **5.81x faster**)  | `29.22 us` (🚀 **6.32x faster**)   |
| **`10000000`** | `454.46 ms` (✅ **1.00x**) | `454.06 ms` (✅ **1.00x faster**) | `217.10 ms` (🚀 **2.09x faster**) | `217.97 ms` (🚀 **2.08x faster**)  |

### Access

This benchmarks accessing each list structure *without bounds checks*.

The structures provided by this crate are expected to perform slightly worse
than the `Vec` equivalents as they require two accesses (one for metadata,
and one for the actual data), while the `Vec` equivalents only need one.

|                | `Vec<String>`           | `Vec<Vec<u8>>`                 | `CompactStrings`               | `CompactBytestrings`            |
|:---------------|:------------------------|:-------------------------------|:-------------------------------|:------------------------------- |
| **`100`**      | `1.13 ns` (✅ **1.00x**) | `1.13 ns` (✅ **1.00x slower**) | `1.24 ns` (✅ **1.09x slower**) | `1.23 ns` (✅ **1.08x slower**)  |
| **`10000`**    | `1.15 ns` (✅ **1.00x**) | `1.14 ns` (✅ **1.00x faster**) | `1.22 ns` (✅ **1.06x slower**) | `1.25 ns` (✅ **1.09x slower**)  |
| **`10000000`** | `1.14 ns` (✅ **1.00x**) | `1.11 ns` (✅ **1.03x faster**) | `1.19 ns` (✅ **1.05x slower**) | `1.20 ns` (✅ **1.05x slower**)  |

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
| **`100`**      | `43.33 ns` (✅ **1.00x**) | `44.00 ns` (✅ **1.02x slower**) | `116.45 ns` (❌ *2.69x slower*)   | `116.64 ns` (❌ *2.69x slower*)    |
| **`10000`**    | `3.22 us` (✅ **1.00x**)  | `3.41 us` (✅ **1.06x slower**)  | `13.31 us` (❌ *4.14x slower*)    | `13.14 us` (❌ *4.09x slower*)     |
| **`10000000`** | `18.18 ms` (✅ **1.00x**) | `19.72 ms` (✅ **1.08x slower**) | `70.70 ms` (❌ *3.89x slower*)    | `64.84 ms` (❌ *3.57x slower*)     |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

