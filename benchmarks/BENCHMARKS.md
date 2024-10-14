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
favourable results for some of the data structures being benchmarked.
When post-fixed with "-hello", all elements are "hello" - which is able
to fit in the 7 bytes of inline storage in the FixedCompact\* structures,
when post-ixed with "-sphinx", all elements are "sphinx of black quartz, 
judge my vow" - which are too long to fit into the 7 bytes of inline
storage.

Should you have ideas on how to better benchmark this, a pull request
would be greatly appreciated.

Row values, if any, indicate the number of (byte)strings in the list.

There may be performance implications, as the `Vec` structures likely
prefer larger (byte)strings while the chosen (byte)string is relatively small.

## Benchmark Results

### Populate

This benchmarks the population of each list structure at various sizes.

The structures provided by this crate are expected to perform better than
the `Vec` equivalents as the data vector will grow rapidly in size to
accomodate for large numbers of new (byte)strings while the `Vec` equivalents
need to find and allocate new space for each new (byte)string.

|                | `Vec<String>-hello`          | `Vec<Vec<u8>>-hello`             | `CompactStrings-hello`           | `FixedCompactStrings-hello`          | `CompactBytestrings-hello`          | `FixedCompactBytestrings-hello`          | `Vec<String>-sphinx`             | `Vec<Vec<u8>>-sphinx`            | `CompactStrings-sphinx`          | `FixedCompactStrings-sphinx`          | `CompactBytestrings-sphinx`          | `FixedCompactBytestrings-sphinx`           |
|:---------------|:-----------------------------|:---------------------------------|:---------------------------------|:-------------------------------------|:------------------------------------|:-----------------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:--------------------------------------|:-------------------------------------|:------------------------------------------ |
| **`100`**      | `2.47 us` (✅ **1.00x**)      | `2.53 us` (✅ **1.02x slower**)   | `446.49 ns` (🚀 **5.52x faster**) | `346.94 ns` (🚀 **7.11x faster**)     | `451.23 ns` (🚀 **5.47x faster**)    | `299.34 ns` (🚀 **8.24x faster**)         | `2.42 us` (✅ **1.02x faster**)   | `2.39 us` (✅ **1.03x faster**)   | `482.01 ns` (🚀 **5.12x faster**) | `439.26 ns` (🚀 **5.61x faster**)      | `471.08 ns` (🚀 **5.24x faster**)     | `448.32 ns` (🚀 **5.50x faster**)           |
| **`10000`**    | `200.03 us` (✅ **1.00x**)    | `195.22 us` (✅ **1.02x faster**) | `28.01 us` (🚀 **7.14x faster**)  | `33.75 us` (🚀 **5.93x faster**)      | `30.18 us` (🚀 **6.63x faster**)     | `29.06 us` (🚀 **6.88x faster**)          | `189.76 us` (✅ **1.05x faster**) | `187.05 us` (✅ **1.07x faster**) | `33.65 us` (🚀 **5.94x faster**)  | `27.89 us` (🚀 **7.17x faster**)       | `33.79 us` (🚀 **5.92x faster**)      | `29.87 us` (🚀 **6.70x faster**)            |
| **`10000000`** | `280.55 ms` (✅ **1.00x**)    | `276.64 ms` (✅ **1.01x faster**) | `49.27 ms` (🚀 **5.69x faster**)  | `39.64 ms` (🚀 **7.08x faster**)      | `48.74 ms` (🚀 **5.76x faster**)     | `34.96 ms` (🚀 **8.02x faster**)          | `303.62 ms` (✅ **1.08x slower**) | `308.42 ms` (✅ **1.10x slower**) | `83.44 ms` (🚀 **3.36x faster**)  | `68.80 ms` (🚀 **4.08x faster**)       | `83.73 ms` (🚀 **3.35x faster**)      | `70.98 ms` (🚀 **3.95x faster**)            |

### Access

This benchmarks accessing each list structure *without bounds checks*.

The structures provided by this crate are expected to perform slightly worse
than the `Vec` equivalents as they require two accesses (one for metadata,
and one for the actual data), while the `Vec` equivalents only need one.

|                | `Vec<String>-hello`          | `Vec<Vec<u8>>-hello`           | `CompactStrings-hello`          | `FixedCompactStrings-hello`          | `CompactBytestrings-hello`          | `FixedCompactBytestrings-hello`          | `Vec<String>-sphinx`           | `Vec<Vec<u8>>-sphinx`          | `CompactStrings-sphinx`          | `FixedCompactStrings-sphinx`          | `CompactBytestrings-sphinx`          | `FixedCompactBytestrings-sphinx`           |
|:---------------|:-----------------------------|:-------------------------------|:--------------------------------|:-------------------------------------|:------------------------------------|:-----------------------------------------|:-------------------------------|:-------------------------------|:---------------------------------|:--------------------------------------|:-------------------------------------|:------------------------------------------ |
| **`100`**      | `1.04 ns` (✅ **1.00x**)      | `1.06 ns` (✅ **1.02x slower**) | `1.17 ns` (❌ *1.12x slower*)    | `1.00 ns` (✅ **1.04x faster**)       | `1.19 ns` (❌ *1.14x slower*)        | `1.01 ns` (✅ **1.03x faster**)           | `1.04 ns` (✅ **1.00x slower**) | `1.05 ns` (✅ **1.01x slower**) | `1.17 ns` (❌ *1.12x slower*)     | `1.55 ns` (❌ *1.48x slower*)          | `1.17 ns` (❌ *1.12x slower*)         | `1.55 ns` (❌ *1.49x slower*)               |
| **`10000`**    | `1.08 ns` (✅ **1.00x**)      | `1.06 ns` (✅ **1.02x faster**) | `1.15 ns` (✅ **1.07x slower**)  | `1.03 ns` (✅ **1.05x faster**)       | `1.18 ns` (✅ **1.09x slower**)      | `1.00 ns` (✅ **1.07x faster**)           | `1.04 ns` (✅ **1.03x faster**) | `1.06 ns` (✅ **1.02x faster**) | `1.17 ns` (✅ **1.09x slower**)   | `1.56 ns` (❌ *1.45x slower*)          | `1.16 ns` (✅ **1.08x slower**)       | `1.57 ns` (❌ *1.46x slower*)               |
| **`10000000`** | `1.04 ns` (✅ **1.00x**)      | `1.08 ns` (✅ **1.04x slower**) | `1.16 ns` (❌ *1.12x slower*)    | `1.00 ns` (✅ **1.03x faster**)       | `1.16 ns` (❌ *1.11x slower*)        | `1.01 ns` (✅ **1.03x faster**)           | `1.08 ns` (✅ **1.04x slower**) | `1.07 ns` (✅ **1.03x slower**) | `1.17 ns` (❌ *1.12x slower*)     | `1.54 ns` (❌ *1.48x slower*)          | `1.17 ns` (❌ *1.12x slower*)         | `1.55 ns` (❌ *1.50x slower*)               |

### Iterate

This benchmarks iterating over each list structure.

The structures provided by this crate are expected to perform worse
than the `Vec` equivalents as they require two accesses (one for metadata,
and one for the actual data), while the `Vec` equivalents only need one.

|                | `Vec<String>-hello`          | `Vec<Vec<u8>>-hello`            | `CompactStrings-hello`          | `FixedCompactStrings-hello`          | `CompactBytestrings-hello`          | `FixedCompactBytestrings-hello`          | `Vec<String>-sphinx`            | `Vec<Vec<u8>>-sphinx`           | `CompactStrings-sphinx`          | `FixedCompactStrings-sphinx`          | `CompactBytestrings-sphinx`          | `FixedCompactBytestrings-sphinx`           |
|:---------------|:-----------------------------|:--------------------------------|:--------------------------------|:-------------------------------------|:------------------------------------|:-----------------------------------------|:--------------------------------|:--------------------------------|:---------------------------------|:--------------------------------------|:-------------------------------------|:------------------------------------------ |
| **`100`**      | `24.64 ns` (✅ **1.00x**)     | `25.03 ns` (✅ **1.02x slower**) | `34.33 ns` (❌ *1.39x slower*)   | `63.64 ns` (❌ *2.58x slower*)        | `34.05 ns` (❌ *1.38x slower*)       | `63.79 ns` (❌ *2.59x slower*)            | `24.17 ns` (✅ **1.02x faster**) | `24.34 ns` (✅ **1.01x faster**) | `33.06 ns` (❌ *1.34x slower*)    | `98.54 ns` (❌ *4.00x slower*)         | `32.54 ns` (❌ *1.32x slower*)        | `103.11 ns` (❌ *4.18x slower*)             |
| **`10000`**    | `2.36 us` (✅ **1.00x**)      | `2.37 us` (✅ **1.00x slower**)  | `4.09 us` (❌ *1.73x slower*)    | `5.75 us` (❌ *2.44x slower*)         | `4.09 us` (❌ *1.73x slower*)        | `5.73 us` (❌ *2.43x slower*)             | `2.40 us` (✅ **1.02x slower**)  | `2.39 us` (✅ **1.01x slower**)  | `4.12 us` (❌ *1.75x slower*)     | `9.81 us` (❌ *4.16x slower*)          | `4.14 us` (❌ *1.76x slower*)         | `9.94 us` (❌ *4.22x slower*)               |
| **`10000000`** | `2.38 ms` (✅ **1.00x**)      | `2.35 ms` (✅ **1.01x faster**)  | `7.25 ms` (❌ *3.05x slower*)    | `6.24 ms` (❌ *2.62x slower*)         | `7.40 ms` (❌ *3.11x slower*)        | `6.27 ms` (❌ *2.64x slower*)             | `2.35 ms` (✅ **1.01x faster**)  | `2.35 ms` (✅ **1.01x faster**)  | `7.40 ms` (❌ *3.11x slower*)     | `9.95 ms` (❌ *4.18x slower*)          | `7.29 ms` (❌ *3.07x slower*)         | `9.96 ms` (❌ *4.19x slower*)               |

### Remove First Element

This benchmarks removing the first element of each list structure.

The structures provided by this crate are expected to perform *significantly* worse
on this benchmark as all (byte)strings stored after the removed element need to
be shifted, causing a large reallocation.

The non-fixed structures provide the `ignore` method which only removes the metadata to combat
this at the cost of stale memory.

This benchmark is intentionally biased against the structures provided by this crate
in order to demonstrate pathological behaviour.

|                | `Vec<String>-hello`          | `Vec<Vec<u8>>-hello`            | `in CompactStrings-hello`          | `in FixedCompactStrings-hello`          | `CompactBytestrings-hello`          | `FixedCompactBytestrings-hello`          | `Vec<String>-sphinx`            | `Vec<Vec<u8>>-sphinx`           | `in CompactStrings-sphinx`          | `in FixedCompactStrings-sphinx`          | `CompactBytestrings-sphinx`          | `FixedCompactBytestrings-sphinx`           |
|:---------------|:-----------------------------|:--------------------------------|:-----------------------------------|:----------------------------------------|:------------------------------------|:-----------------------------------------|:--------------------------------|:--------------------------------|:------------------------------------|:-----------------------------------------|:-------------------------------------|:------------------------------------------ |
| **`100`**      | `30.12 ns` (✅ **1.00x**)     | `29.99 ns` (✅ **1.00x faster**) | `84.86 ns` (❌ *2.82x slower*)      | `90.36 ns` (❌ *3.00x slower*)           | `87.61 ns` (❌ *2.91x slower*)       | `90.02 ns` (❌ *2.99x slower*)            | `29.77 ns` (✅ **1.01x faster**) | `29.64 ns` (✅ **1.02x faster**) | `88.67 ns` (❌ *2.94x slower*)       | `88.08 ns` (❌ *2.92x slower*)            | `90.55 ns` (❌ *3.01x slower*)        | `89.54 ns` (❌ *2.97x slower*)              |
| **`10000`**    | `2.84 us` (✅ **1.00x**)      | `2.83 us` (✅ **1.01x faster**)  | `11.70 us` (❌ *4.11x slower*)      | `10.32 us` (❌ *3.63x slower*)           | `12.01 us` (❌ *4.23x slower*)       | `10.56 us` (❌ *3.72x slower*)            | `2.89 us` (✅ **1.02x slower**)  | `2.83 us` (✅ **1.01x faster**)  | `11.56 us` (❌ *4.06x slower*)       | `10.63 us` (❌ *3.74x slower*)            | `11.64 us` (❌ *4.09x slower*)        | `9.97 us` (❌ *3.51x slower*)               |
| **`10000000`** | `16.97 ms` (✅ **1.00x**)     | `18.45 ms` (✅ **1.09x slower**) | `29.07 ms` (❌ *1.71x slower*)      | `5.32 ms` (🚀 **3.19x faster**)          | `28.67 ms` (❌ *1.69x slower*)       | `5.26 ms` (🚀 **3.23x faster**)           | `18.32 ms` (✅ **1.08x slower**) | `18.53 ms` (✅ **1.09x slower**) | `50.67 ms` (❌ *2.99x slower*)       | `38.37 ms` (❌ *2.26x slower*)            | `51.35 ms` (❌ *3.02x slower*)        | `39.01 ms` (❌ *2.30x slower*)              |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

