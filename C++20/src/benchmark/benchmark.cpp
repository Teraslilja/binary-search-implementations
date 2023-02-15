#include <benchmark/benchmark.h>

#include <iostream>

#include "helper.hpp"

//
// Configuration for benchmarks
//

using DataType = std::int32_t;
using IndexType = std::size_t;

static std::size_t constexpr bench_size = 1u << 16u;

//
// Setups for benchmarks
//

static inline std::vector<DataType> const BM_setup(std::size_t const N)
{
    return filler<DataType, IndexType>(N);
}

template <std::size_t N>
static inline std::array<DataType, N> constexpr BM_setup2()
{
    return filler2<DataType, IndexType, N>();
}

//
// Common testcases for benchmarks
//

template <class BS, char const* NAME>
static inline void BM_dynamic_testcase(benchmark::State& state)
{
    std::vector<DataType> const bench = BM_setup(bench_size);
    for (auto _ : state) {
        if (!test(BS {}, bench)) {
            std::cerr << NAME << " failed!" << std::endl;
            std::exit(EXIT_FAILURE);
        }
    }
}

template <class BS, char const* NAME>
static inline void BM_static_testcase(benchmark::State& state)
{
    std::array<DataType, bench_size> const bench = BM_setup2<bench_size>();
    for (auto _ : state) {
        if (!test(BS {}, bench)) {
            std::cerr << NAME << " failed!" << std::endl;
            std::exit(EXIT_FAILURE);
        }
    }
}

template <class BS, char const* NAME>
static inline void BM_dynamic_testcase_indexless(benchmark::State& state)
{
    std::vector<DataType> const bench = BM_setup(bench_size);
    for (auto _ : state) {
        if (!test_indexless(BS {}, bench)) {
            std::cerr << NAME << " failed!" << std::endl;
            std::exit(EXIT_FAILURE);
        }
    }
}

template <class BS, char const* NAME>
static inline void BM_static_testcase_indexless(benchmark::State& state)
{
    std::array<DataType, bench_size> const bench = BM_setup2<bench_size>();
    for (auto _ : state) {
        if (!test_indexless(BS {}, bench)) {
            std::cerr << NAME << " failed!" << std::endl;
            std::exit(EXIT_FAILURE);
        }
    }
}

//
// Benchmarks
//

static void BM_traditional1(benchmark::State& state)
{
    static char constexpr name[] = "traditional1";
    BM_dynamic_testcase_indexless<traditional1<DataType>, name>(state);
}

static void BM_traditional2(benchmark::State& state)
{
    static char constexpr name[] = "traditional2";
    BM_dynamic_testcase_indexless<traditional2<DataType>, name>(state);
}

static void BM_alternative(benchmark::State& state)
{
    static char constexpr name[] = "alternative";
    BM_dynamic_testcase_indexless<alternative<DataType>, name>(state);
}

static void BM_range(benchmark::State& state)
{
    static char constexpr name[] = "range";
    BM_dynamic_testcase_indexless<range<DataType>, name>(state);
}

static void BM_power_dynamic(benchmark::State& state)
{
    static char constexpr name[] = "power<dynamic>";
    BM_dynamic_testcase_indexless<power<DataType>, name>(state);
}

static void BM_power_static(benchmark::State& state)
{
    static char constexpr name[] = "power<static>";
    BM_static_testcase_indexless<power<DataType>, name>(state);
}

BENCHMARK(BM_traditional1);
BENCHMARK(BM_traditional2);
BENCHMARK(BM_alternative);
BENCHMARK(BM_range);
BENCHMARK(BM_power_dynamic);
BENCHMARK(BM_power_static);
