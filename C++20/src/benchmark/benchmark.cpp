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

// Define the base based on std::binary_search
template <typename D, typename I = std::size_t>
requires std::is_unsigned_v<I> && std::is_integral_v<I>
struct baseline : public binary_search<D, I> {
public:
    using data_t = binary_search<D, I>::data_t;
    using index_t = binary_search<D, I>::index_t;

    [[nodiscard]] std::optional<index_t> constexpr impl(std::span<data_t const> const data,
        data_t const v) const noexcept override
    {
        return (((data.size() > I(0)) && std::binary_search(data.begin(), data.end(), v)))
            ? std::optional(0) // Index is not known
            : std::nullopt;
    }

    template <std::size_t N>
    [[nodiscard]] inline std::optional<index_t> constexpr impl(std::array<data_t const, N> const data,
        data_t const v) const noexcept
    {
        return impl(std::span(data), v);
    }
};

static void BM_baseline(benchmark::State& state)
{
    static char constexpr name[] = "baseline";
    BM_dynamic_testcase_indexless<baseline<DataType>, name>(state);
}

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

BENCHMARK(BM_baseline);
BENCHMARK(BM_traditional1);
BENCHMARK(BM_traditional2);
BENCHMARK(BM_alternative);
BENCHMARK(BM_range);
BENCHMARK(BM_power_dynamic);
BENCHMARK(BM_power_static);
