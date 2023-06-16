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

|        Bench Name        |   Lower   | Estimate |  Upper  |
|--------------------------|-----------|----------|---------|
|populate vec 20           |231.29 ns  |231.46 ns |231.65 ns|
|populate compact 20       |199.33 ns  |200.02 ns |200.42 ns|
|populate vec 100          |2.5393 µs  |2.6207 µs |2.7257 µs|
|populate compact 100      |479.54 ns  |480.22 ns |480.65 ns|
|populate vec 1000         |20.662 µs  |20.756 µs |20.858 µs|
|populate compact 1000     |2.9085 µs  |2.9140 µs |2.9177 µs|
|populate vec 10000        |192.99 µs  |193.26 µs |193.53 µs|
|populate compact 10000    |26.421 µs  |26.444 µs |26.469 µs|
|populate vec 100000       |3.5896 ms  |3.6342 ms |3.6634 ms|
|populate compact 100000   |268.18 µs  |268.34 µs |268.51 µs|
|populate vec 1000000      |47.932 ms  |48.047 ms |48.153 ms|
|populate compact 1000000  |11.070 ms  |11.096 ms |11.122 ms|
|populate vec 10000000     |386.44 ms  |387.06 ms |387.71 ms|
|populate compact 10000000 |114.67 ms  |119.57 ms |124.32 ms|
|access vec                |1.1185 ns  |1.1284 ns |1.1402 ns|
|access compact            |1.1892 ns  |1.1929 ns |1.2007 ns|
|remove first vec          |3.1943 µs  |3.2097 µs |3.2301 µs|
|remove first compact      |12.847 µs  |12.859 µs |12.878 µs|
|remove last vec           |10.978 ns  |11.028 ns |11.096 ns|
|remove last compact       |7.2972 ns  |7.3158 ns |7.3261 ns|

Benchmarks run on a laptop with an AMD Ryzen™ 7 5800H Mobile Processor @ 3.2GHz
