# binary-search-implementations
Several binary search implementations first with C++20 and then Rust


The purpose of this repository is to
1) learn Rust and compare it to C++
2) study challenging, but simple concept of 'binary search'


To Do List:
* Cache friendly Eytzinger array layout, and
* Mimic Google Benchmark

All these binary search implementations check, if given value exists in given sorted array of values and return optional index of location in array. And is ignorant of duplicate values in array.


There are two alternatives for each implementation:
* dynamic - size of array is not a compile time constant
* static - size of array is a compile time constant


The implementations are
* 'traditional_signed' with signed index type, and two conditions at inner loop
  * static implementation uses dynamic implementation
* 'traditional_unsigned' like 'traditional_signed', but unsigned index type
* 'alternative' like 'traditional_unsigned', but single condition at inner loop
* 'range' like 'alternative', but have low & width instead of low & high, and condition is replaced with ternary operator
* 'power' like 'range', but index is updated with bitwise operator instead of add operator
  * static implementation have inner loop with fixed (unrollable) number of rounds

NOTE: WITH UNIT TESTS, ONLY 'power' VERSIONS ARE MANAGED TO PROVE **NOT** TO CONTAIN A SINGLE BUG.


Relative performance with array of 2^16 values (filled with even values, find even and odd values in array's range):
```
Run on (16 X 3600 MHz CPU s)
CPU Caches:
  L1 Data 32 KiB (x8)
  L1 Instruction 32 KiB (x8)
  L2 Unified 512 KiB (x8)
  L3 Unified 16384 KiB (x2)
Load Average: 1.72, 2.31, 1.72
-------------------------------------------------------------------------------------------------------------
Benchmark                                                           Time             CPU   Iterations
-------------------------------------------------------------------------------------------------------------
DynamicFixture/baseline_indexless/65536                       4484932 ns      4484679 ns          157
DynamicFixture/signed_traditional_indexless/65536             3961233 ns      3961175 ns          176
DynamicFixture/unsigned_traditional_indexless/65536           4335896 ns      4335743 ns          161
DynamicFixture/alternative_indexless/65536                    5363226 ns      5362970 ns          129
DynamicFixture/range_indexless/65536                          2630354 ns      2630239 ns          269
DynamicFixture/power_indexless/65536                          2019235 ns      2019152 ns          346
StaticFixture/power_indexless/65536                           1838083 ns      1838041 ns          380
 ..
StaticFixture_65536/power_indexless/repeats:11                1837963 ns      1837916 ns          381
StaticFixture_65536/power_indexless/repeats:11                1838108 ns      1838013 ns          381
StaticFixture_65536/power_indexless/repeats:11                1841964 ns      1841902 ns          381
StaticFixture_65536/power_indexless/repeats:11                1841313 ns      1841172 ns          381
StaticFixture_65536/power_indexless/repeats:11                1846215 ns      1846100 ns          381
StaticFixture_65536/power_indexless/repeats:11                1845245 ns      1845165 ns          381
StaticFixture_65536/power_indexless/repeats:11                1842309 ns      1842241 ns          381
StaticFixture_65536/power_indexless/repeats:11                1847970 ns      1847833 ns          381
StaticFixture_65536/power_indexless/repeats:11                1840506 ns      1840443 ns          381
StaticFixture_65536/power_indexless/repeats:11                1844986 ns      1844907 ns          381
StaticFixture_65536/power_indexless/repeats:11                1844495 ns      1844443 ns          381
StaticFixture_65536/power_indexless/repeats:11_mean           1842825 ns      1842740 ns           11
StaticFixture_65536/power_indexless/repeats:11_median         1842309 ns      1842241 ns           11
StaticFixture_65536/power_indexless/repeats:11_stddev            3253 ns         3240 ns           11
StaticFixture_65536/power_indexless/repeats:11_cv                0.18 %          0.18 %            11
```

Recommended for reading:
* Cannizzo, "A Fast and Vectorizable Alternative to Binary Search in O(1) with Wide Applicability to Arrays of Floating Point Numbers", arxiv.org, 2017. https://arxiv.org/abs/1506.08620
* "What are the pitfalls in implementing binary search?" https://stackoverflow.com/questions/504335/what-are-the-pitfalls-in-implementing-binary-search
* "Overflow Bug in Binary Search" https://medium.com/swlh/overflow-bug-in-binary-search-c4d4a824807a


For building (C++) the following packages must be installed:
* libgmock-dev, libgtest-dev
* libbenchmark-dev, libbenchmark1
* cmake
* g++

For building (Rust) the following web page tells, how rustup is installed:
* https://rustup.rs/

To upgrade Rust installation:
```
rustup update
```


Building and executing C++ code:
```
cd C++20
mkdir build
cd build
cmake ..
make clean all
./binary_search
```

Building and executing Rust code:
```
cd Rust
cargo build
cargo test --bins
cargo run
```

The '--release' option can be used optionally, but some unit tests are expected to fail as debug/release behaviour is different with threads terminated by any signal.
