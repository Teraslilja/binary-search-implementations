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




Eytzinger layout based binary search:
Binary search algorithms that use monotonic arrays, thery basically are accessing arryas in random order, which means that cache hit ratio shall be poor.
In Eytzinger, the array layout is not any more monotonically ordered, which will provide a better temporal locality.

The implementations are
* without cache update hint
* branchless without cache update hint
* branchless with cache update hint



Relative performance with array of 2^16 values (filled with even values, find even and odd values in array's range):
```
Run on (16 X 5465.54 MHz CPU s)
CPU Caches:
  L1 Data 32 KiB (x8)
  L1 Instruction 32 KiB (x8)
  L2 Unified 1024 KiB (x8)
  L3 Unified 32768 KiB (x1)
Load Average: 1.38, 1.97, 1.83
----------------------------------------------------------------------------------------------------------------
Benchmark                                                                      Time             CPU   Iterations
----------------------------------------------------------------------------------------------------------------
DynamicFixture/baseline_indexless/65536                                  2940814 ns      2940761 ns          239
DynamicFixture/signed_traditional_indexless/65536                        3051001 ns      3050950 ns          230
DynamicFixture/unsigned_traditional_indexless/65536                      3156339 ns      3156376 ns          222
DynamicFixture/alternative_indexless/65536                               3458791 ns      3458750 ns          202
DynamicFixture/range_indexless/65536                                     1639913 ns      1639917 ns          426
DynamicFixture/power_indexless/65536                                     1407429 ns      1407437 ns          497
DynamicFixture/eytzinger_hintless_indexless/65536                        1469165 ns      1469148 ns          474
DynamicFixture/eytzinger_branchless_indexless/65536                      1871850 ns      1871845 ns          373
DynamicFixture/eytzinger_prefetching_indexless/65536                     2437523 ns      2437503 ns          287
StaticFixture/power_indexless/65536                                      1079569 ns      1079563 ns          649
StaticFixture/eytzinger_hintless_indexless/65536                         1376517 ns      1376506 ns          509
StaticFixture/eytzinger_branchless_indexless/65536                       1913081 ns      1913074 ns          366
StaticFixture/eytzinger_prefetching_indexless/65536                      2408027 ns      2407991 ns          293
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
