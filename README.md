# binary-search-implementations
Several binary search implementations first with C++20 and then Rust


The purpose of this repository is to
1) learn Rust and compare it to C++
2) study challenging, but simple concept of 'binary search'


To Do List:
* Cache friendly Eytzinger array layout, and
* Mimic Google Benchmark in Rust

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
Run on (16 X 5489.36 MHz CPU s)
CPU Caches:
  L1 Data 32 KiB (x8)
  L1 Instruction 32 KiB (x8)
  L2 Unified 1024 KiB (x8)
  L3 Unified 32768 KiB (x1)
Load Average: 2.73, 1.73, 1.43
-----------------------------------------------------------------------------------------------------
Benchmark                                                           Time             CPU   Iterations
-----------------------------------------------------------------------------------------------------
DynamicFixture/baseline_indexless/65536                       2751419 ns      2751441 ns          256
DynamicFixture/signed_traditional_indexless/65536             3095892 ns      3095913 ns          227
DynamicFixture/unsigned_traditional_indexless/65536           3190163 ns      3190171 ns          219
DynamicFixture/alternative_indexless/65536                    3437417 ns      3437432 ns          203
DynamicFixture/range_indexless/65536                          1645408 ns      1645403 ns          425
DynamicFixture/power_indexless/65536                          1415216 ns      1415210 ns          494
StaticFixture/power_indexless/65536                           1083954 ns      1083947 ns          645
StaticFixture_1/power_indexless/repeats:11                      0.057 ns        0.057 ns   12270390751
StaticFixture_1/power_indexless/repeats:11                      0.057 ns        0.057 ns   12270390751
StaticFixture_1/power_indexless/repeats:11                      0.057 ns        0.057 ns   12270390751
StaticFixture_1/power_indexless/repeats:11                      0.057 ns        0.057 ns   12270390751
StaticFixture_1/power_indexless/repeats:11                      0.057 ns        0.057 ns   12270390751
StaticFixture_1/power_indexless/repeats:11                      0.057 ns        0.057 ns   12270390751
StaticFixture_1/power_indexless/repeats:11                      0.057 ns        0.057 ns   12270390751
StaticFixture_1/power_indexless/repeats:11                      0.057 ns        0.057 ns   12270390751
StaticFixture_1/power_indexless/repeats:11                      0.057 ns        0.057 ns   12270390751
StaticFixture_1/power_indexless/repeats:11                      0.057 ns        0.057 ns   12270390751
StaticFixture_1/power_indexless/repeats:11                      0.058 ns        0.058 ns   12270390751
StaticFixture_1/power_indexless/repeats:11_mean                 0.057 ns        0.057 ns           11
StaticFixture_1/power_indexless/repeats:11_median               0.057 ns        0.057 ns           11
StaticFixture_1/power_indexless/repeats:11_stddev               0.000 ns        0.000 ns           11
StaticFixture_1/power_indexless/repeats:11_cv                    0.43 %          0.42 %            11
```

Recommended for reading:
* Cannizzo, "A Fast and Vectorizable Alternative to Binary Search in O(1) with Wide Applicability to Arrays of Floating Point Numbers", arxiv.org, 2017. https://arxiv.org/abs/1506.08620
* "What are the pitfalls in implementing binary search?" https://stackoverflow.com/questions/504335/what-are-the-pitfalls-in-implementing-binary-search
* "Overflow Bug in Binary Search" https://medium.com/swlh/overflow-bug-in-binary-search-c4d4a824807a


For building (C++) the following packages must be installed:
* libgmock-dev, libgtest-dev
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
