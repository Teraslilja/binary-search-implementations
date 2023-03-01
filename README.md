# binary-search-implementations
Several binary search implementations first with C++20 and then Rust


The purpose of this repository is to
1) learn Rust and compare it to C++
2) study challenging, but simple concept of 'binary search'


To Do List:
* Use prefetching,
* Cache friendly Eytzinger array layout, and
* Mimic Google Benchmark

All these binary search implementations check, if given value exists in given sorted array of values and return optional index of location in array. And is ignorant of duplicate values in array.


There are two alternatives for each implementation:
* dynamic - size of array is not a compile time constant
* static - size of array is a compile time constant


The implementations are
* 'traditional1' with signed index type, and two conditions at inner loop
  * static implementation uses dynamic implementation
* 'traditional2' like 'traditional1', but unsigned index type
* 'alternative' like 'traditional2', but single condition at inner loop
* 'range' like 'alternative', but have low & width instead of low & high, and condition is replaced with ternary operator
* 'power' like 'range', but index is updated with bitwise operator instead of add operator
  * static implementation have inner loop with fixed (unrollable) number of rounds

NOTE: WITH UNIT TESTS, ONLY 'power' VERSIONS ARE MANAGED TO PROVE **NOT** TO CONTAIN A SINGLE BUG.


Relative performance with array of 2^16 values (filled with even values, find even and odd values in array's range):
```
Run on (16 X 4426.17 MHz CPU s)
CPU Caches:
  L1 Data 32 KiB (x8)
  L1 Instruction 32 KiB (x8)
  L2 Unified 512 KiB (x8)
  L3 Unified 16384 KiB (x2)
***WARNING*** CPU scaling is enabled, the benchmark real time measurements may be noisy and will incur extra overhead.
-----------------------------------------------------------
Benchmark                 Time             CPU   Iterations
-----------------------------------------------------------
BM_baseline         4650985 ns      4650966 ns          151
BM_traditional1     4093059 ns      4092944 ns          170
BM_traditional2     4421060 ns      4420952 ns          157
BM_alternative      4192862 ns      4192723 ns          167
BM_range            2640873 ns      2640748 ns          264
BM_power_dynamic    2086927 ns      2086931 ns          342
BM_power_static     1826986 ns      1826973 ns          384
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

For building (Rust) the following wegb page tels, how rustup is installed:
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

The '--release' option can be used optionally, but some unit tests are expected to fail as debug/release behaviour is differetn with threads terminated with any signal.
