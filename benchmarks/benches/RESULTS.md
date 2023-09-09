# Benchmark Results

Only a few operations that are thought to have significant differences in
performance from `Vec<String>` are benchmarked and shown here.

The benchmark itself uses quite naive testing methods and are by no means
perfect. For example, the removal benchmarks also include the timing to
re-add elements to prevent the benchmark from exhausting the elements.

Furthermore, all the elements are of the same length and may lead to
favourable results for one of the data structures being benchmarked.

Should you have ideas on how to better benchmark this, a pull request
would be greatly appreciated.

|              Bench Name              |   Lower   | Estimate  |   Upper   |
|--------------------------------------|-----------|-----------|-----------|
|populate str vec 100                  | 2.5602 µs | 2.5732 µs | 2.5844 µs |
|populate bytestring vec 100           | 2.5488 µs | 2.5540 µs | 2.5650 µs |
|populate compact strings 100          | 577.44 ns | 584.26 ns | 594.42 ns |
|populate compact bytestrings 100      | 527.75 ns | 528.13 ns | 528.64 ns |
|populate str vec 10000                | 183.78 µs | 185.77 µs | 190.41 µs |
|populate bytestring vec 10000         | 185.88 µs | 186.03 µs | 186.15 µs |
|populate compact strings 10000        | 32.061 µs | 32.149 µs | 32.226 µs |
|populate compact bytestrings 10000    | 29.118 µs | 29.153 µs | 29.193 µs |
|populate str vec 10000000             | 425.44 ms | 430.63 ms | 439.21 ms |
|populate bytestring vec 10000000      | 402.99 ms | 420.51 ms | 439.29 ms |
|populate compact strings 10000000     | 121.65 ms | 122.85 ms | 124.68 ms |
|populate compact bytestrings 10000000 | 120.83 ms | 123.71 ms | 127.51 ms |
|access vec strings                    | 1.1256 ns | 1.1286 ns | 1.1319 ns |
|remove first vec string               | 21.558 ms | 23.087 ms | 26.311 ms |
|access vec bytestrings                | 1.1317 ns | 1.1348 ns | 1.1394 ns |
|remove first vec bytestring           | 28.523 ms | 29.682 ms | 30.745 ms |
|access compact strings                | 1.2486 ns | 1.2584 ns | 1.2773 ns |
|remove first compact string           | 73.958 ms | 74.853 ms | 75.728 ms |
|access compact bytestrings            | 1.2396 ns | 1.2465 ns | 1.2557 ns |
|remove first compact bytestring       | 90.946 ms | 93.435 ms | 97.069 ms |


Benchmarks run on a laptop with an AMD Ryzen™ 7 5800H Mobile Processor @ 3.2GHz
