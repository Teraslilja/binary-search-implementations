#include <benchmark/benchmark.h>

#include <iostream>

#include "helper.hpp"

//
// Configuration for benchmarks
//

using DataType = std::int32_t;
using IndexType = std::size_t;

static std::size_t constexpr MAX_BENCH_SIZE = 1u << 16u;
static size_t constexpr REPEATIONS = 11u;

// clang-format off
// Static test sizes
#define APPLY_MACRO(M, OP) \
    M(1) OP;               \
    M(2) OP;               \
    M(4) OP;               \
    M(8) OP;               \
    M(16) OP;              \
    M(32) OP;              \
    M(64) OP;              \
    M(128) OP;             \
    M(256) OP;             \
    M(512) OP;             \
    M(1024) OP;            \
    M(2048) OP;            \
    M(4096) OP;            \
    M(8192) OP;            \
    M(16384) OP;           \
    M(32768) OP;           \
    M(65536) OP;
// clang-format on

//
// Define the baseline based on std::binary_search
//
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
            ? std::make_optional(0) // Index is not known
            : std::nullopt;
    }

    template <std::size_t N>
    [[nodiscard]] inline std::optional<index_t> constexpr impl(std::array<data_t const, N> const data,
        data_t const v) const noexcept
    {
        return impl(std::span(data), v);
    }
};

//
// Fixtures for benchmarks
//
class DynamicFixture : public benchmark::Fixture {
public:
    void SetUp(::benchmark::State& state)
    {
        if (this->ArgsCnt() <= 0) {
            state.SkipWithError("No test size defined!");
            return;
        }

        int64_t const n = state.range();
        std::size_t const N = static_cast<std::size_t>(n);
        this->bench_monotonic_ = filler<DataType, IndexType>(N);
        this->bench_eytzinger_.resize(this->bench_monotonic_.size());
        [[maybe_unused]] bool const status = Helpers::eytzinger_layout(std::span(this->bench_eytzinger_.begin(),
                                                                           this->bench_eytzinger_.size()),
            std::span(this->bench_monotonic_.cbegin(),
                this->bench_monotonic_.size()));
    }

    void TearDown(::benchmark::State& state)
    {
        (void)state;
    }

    template <class BS, char const* NAME>
    inline void testcase_monotonic_indexed(benchmark::State const& state)
    {
        (void)state;
        if (!test(BS {}, this->bench_monotonic_)) {
            std::cerr << NAME << " failed!" << std::endl;
            std::exit(EXIT_FAILURE);
        }
    }

    template <class BS, char const* NAME>
    inline void testcase_monotonic_indexless(benchmark::State const& state)
    {
        (void)state;
        if (!test_indexless(BS {}, this->bench_monotonic_)) {
            std::cerr << NAME << " failed!" << std::endl;
            std::exit(EXIT_FAILURE);
        }
    }

    template <class BS, char const* NAME>
    inline void testcase_eytzinger_indexless(benchmark::State const& state)
    {
        (void)state;
        if (!test_indexless(BS {}, this->bench_eytzinger_, this->bench_monotonic_)) {
            std::cerr << NAME << " failed!" << std::endl;
            std::exit(EXIT_FAILURE);
        }
    }

protected:
    std::vector<DataType> bench_monotonic_;
    std::vector<DataType> bench_eytzinger_;
};

template <std::size_t N>
class StaticFixtureBase : public benchmark::Fixture {
public:
    void SetUp(::benchmark::State& state)
    {
        (void)state;
        this->bench_monotonic_ = filler2<DataType, IndexType, N>();
        [[maybe_unused]] bool const status = Helpers::eytzinger_layout(std::span(this->bench_eytzinger_.begin(),
                                                                           this->bench_eytzinger_.size()),
            std::span(this->bench_monotonic_.cbegin(),
                this->bench_monotonic_.size()));
    }

    void TearDown(::benchmark::State& state)
    {
        (void)state;
    }

    template <class BS, char const* NAME>
    inline void testcase_indexed(benchmark::State const& state)
    {
        (void)state;
        if (!test(BS {}, this->bench_monotonic_)) {
            std::cerr << NAME << " failed!" << std::endl;
            std::exit(EXIT_FAILURE);
        }
    }

    template <class BS, char const* NAME>
    inline void testcase_monotonic_indexless(benchmark::State const& state)
    {
        (void)state;
        if (!test_indexless(BS {}, this->bench_monotonic_)) {
            std::cerr << NAME << " failed!" << std::endl;
            std::exit(EXIT_FAILURE);
        }
    }

    template <class BS, char const* NAME>
    inline void testcase_eytzinger_indexless(benchmark::State const& state)
    {
        (void)state;
        if (!test_indexless(BS {}, this->bench_eytzinger_, this->bench_monotonic_)) {
            std::cerr << NAME << " failed!" << std::endl;
            std::exit(EXIT_FAILURE);
        }
    }

protected:
    alignas(CACHE_LINE_SIZE) std::array<DataType, N> bench_monotonic_;
    alignas(CACHE_LINE_SIZE) std::array<DataType, N> bench_eytzinger_;
};

extern "C" bool dynamic_binary_search_power(size_t const N, int32_t const data[], int32_t const value);

class C_Fixture : public benchmark::Fixture {
public:
    void SetUp(::benchmark::State& state)
    {
        if (this->ArgsCnt() <= 0) {
            state.SkipWithError("No test size defined!");
            return;
        }

        int64_t const n = state.range();
        std::size_t const N = static_cast<std::size_t>(n);
        this->bench_ = filler<DataType, IndexType>(N);
    }

    void TearDown(::benchmark::State& state)
    {
        (void)state;
    }

    template <class BS, char const* NAME>
    inline void search_first(benchmark::State const& state)
    {
        (void)state;
        BS bs {};
        bool const result = dynamic_binary_search_power(bench_.size(), bench_.data(), bench_.front());
        assert(result);
        (void)result;
    }

    template <class BS, char const* NAME>
    inline void search_last(benchmark::State const& state)
    {
        (void)state;
        BS bs {};
        bool const result = dynamic_binary_search_power(bench_.size(), bench_.data(), bench_.back());
        assert(result);
        (void)result;
    }

    template <class BS, char const* NAME>
    inline void fail_first(benchmark::State const& state)
    {
        (void)state;
        BS bs {};
        bool const result = dynamic_binary_search_power(bench_.size(), bench_.data(), bench_.front() - 1);
        assert(!result);
        (void)result;
    }

    template <class BS, char const* NAME>
    inline void fail_last(benchmark::State const& state)
    {
        (void)state;
        BS bs {};
        bool const result = dynamic_binary_search_power(bench_.size(), bench_.data(), bench_.back() + 1);
        assert(!result);
        (void)result;
    }

protected:
    std::vector<DataType> bench_;
};

#define STATIC_FIXTURE_NAME(NAME) StaticFixture##NAME

#define STATIC_DEFINE_FIXTURE(NAME, N) \
    class STATIC_FIXTURE_NAME(NAME)    \
        : public StaticFixtureBase<N> { }

STATIC_DEFINE_FIXTURE(, MAX_BENCH_SIZE);

#define STATIC_DEFINE_FIXTURE_NAME_COUNT(N) STATIC_DEFINE_FIXTURE(_##N, N)

APPLY_MACRO(STATIC_DEFINE_FIXTURE_NAME_COUNT, )

//
// Define benchmarks for fixtures
//

// Dynamic
BENCHMARK_DEFINE_F(DynamicFixture, baseline_indexless)
(benchmark::State& state)
{
    static char constexpr name[] = "baseline";
    for (auto _ : state)
        this->testcase_monotonic_indexless<baseline<DataType>, name>(state);
}

BENCHMARK_DEFINE_F(DynamicFixture, signed_traditional_indexless)
(benchmark::State& state)
{
    static char constexpr name[] = "signed_traditional";
    for (auto _ : state)
        this->testcase_monotonic_indexless<signed_traditional<DataType>, name>(state);
}

BENCHMARK_DEFINE_F(DynamicFixture, unsigned_traditional_indexless)
(benchmark::State& state)
{
    static char constexpr name[] = "unsigned_traditional";
    for (auto _ : state)
        this->testcase_monotonic_indexless<unsigned_traditional<DataType>, name>(state);
}

BENCHMARK_DEFINE_F(DynamicFixture, alternative_indexless)
(benchmark::State& state)
{
    static char constexpr name[] = "alternative";
    for (auto _ : state)
        this->testcase_monotonic_indexless<alternative<DataType>, name>(state);
}

BENCHMARK_DEFINE_F(DynamicFixture, range_indexless)
(benchmark::State& state)
{
    static char constexpr name[] = "range";
    for (auto _ : state)
        this->testcase_monotonic_indexless<range<DataType>, name>(state);
}

BENCHMARK_DEFINE_F(DynamicFixture, power_indexless)
(benchmark::State& state)
{
    static char constexpr name[] = "power<dynamic>";
    for (auto _ : state)
        this->testcase_monotonic_indexless<power<DataType>, name>(state);
}

BENCHMARK_DEFINE_F(DynamicFixture, eytzinger_hintless_indexless)
(benchmark::State& state)
{
    static char constexpr name[] = "eytzinger_hintless";
    for (auto _ : state)
        this->testcase_eytzinger_indexless<eytzinger_hintless<DataType>, name>(state);
}

BENCHMARK_DEFINE_F(DynamicFixture, eytzinger_branchless_indexless)
(benchmark::State& state)
{
    static char constexpr name[] = "eytzinger_branchless";
    for (auto _ : state)
        this->testcase_eytzinger_indexless<eytzinger_branchless<DataType>, name>(state);
}

BENCHMARK_DEFINE_F(DynamicFixture, eytzinger_prefetching_indexless)
(benchmark::State& state)
{
    static char constexpr name[] = "eytzinger_prefetching";
    for (auto _ : state)
        this->testcase_eytzinger_indexless<eytzinger_prefetching<DataType>, name>(state);
}

// Static
#define STATIC_FIXTURE_DEFINE_TEST(NAME, N)                                                   \
    BENCHMARK_DEFINE_F(STATIC_FIXTURE_NAME(NAME), power_indexless)                            \
    (benchmark::State & state)                                                                \
    {                                                                                         \
        static char constexpr name[] = "power<static," #N ">";                                \
        for (auto _ : state)                                                                  \
            this->testcase_monotonic_indexless<power<DataType>, name>(state);                 \
    }                                                                                         \
    BENCHMARK_DEFINE_F(STATIC_FIXTURE_NAME(NAME), eytzinger_hintless_indexless)               \
    (benchmark::State & state)                                                                \
    {                                                                                         \
        static char constexpr name[] = "eytzinger_hintless<static," #N ">";                   \
        for (auto _ : state)                                                                  \
            this->testcase_eytzinger_indexless<eytzinger_hintless<DataType>, name>(state);    \
    }                                                                                         \
    BENCHMARK_DEFINE_F(STATIC_FIXTURE_NAME(NAME), eytzinger_branchless_indexless)             \
    (benchmark::State & state)                                                                \
    {                                                                                         \
        static char constexpr name[] = "eytzinger_branchless<static," #N ">";                 \
        for (auto _ : state)                                                                  \
            this->testcase_eytzinger_indexless<eytzinger_branchless<DataType>, name>(state);  \
    }                                                                                         \
    BENCHMARK_DEFINE_F(STATIC_FIXTURE_NAME(NAME), eytzinger_prefetching_indexless)            \
    (benchmark::State & state)                                                                \
    {                                                                                         \
        static char constexpr name[] = "eytzinger_prefetching<static," #N ">";                \
        for (auto _ : state)                                                                  \
            this->testcase_eytzinger_indexless<eytzinger_prefetching<DataType>, name>(state); \
    }

STATIC_FIXTURE_DEFINE_TEST(, MAX_BENCH_SIZE)

#define STATIC_FIXTURE_DEFINE_TEST_NAME_COUNT(N) STATIC_FIXTURE_DEFINE_TEST(_##N, N)

APPLY_MACRO(STATIC_FIXTURE_DEFINE_TEST_NAME_COUNT, )

// C wrappers
BENCHMARK_DEFINE_F(C_Fixture, search_first)
(benchmark::State& state)
{
    static char constexpr name[] = "C<power<dynamic>>";
    for (auto _ : state)
        this->search_first<power<DataType>, name>(state);
}

BENCHMARK_DEFINE_F(C_Fixture, search_last)
(benchmark::State& state)
{
    static char constexpr name[] = "C<power<dynamic>>";
    for (auto _ : state)
        this->search_last<power<DataType>, name>(state);
}

BENCHMARK_DEFINE_F(C_Fixture, fail_first)
(benchmark::State& state)
{
    static char constexpr name[] = "C<power<dynamic>>";
    for (auto _ : state)
        this->fail_first<power<DataType>, name>(state);
}

BENCHMARK_DEFINE_F(C_Fixture, fail_last)
(benchmark::State& state)
{
    static char constexpr name[] = "C<power<dynamic>>";
    for (auto _ : state)
        this->fail_last<power<DataType>, name>(state);
}

//
// Benchmarks
//
#define STATISTICS(N) Repetitions(N)->ComputeStatistics("min", [](std::vector<double> const& v) -> double { return *std::ranges::min_element(v); })->ComputeStatistics("max", [](std::vector<double> const& v) -> double { return *std::ranges::max_element(v); })
#define REPEAT_FROM_1_TO(N) RangeMultiplier(4u)->Range(1u, N)

BENCHMARK_REGISTER_F(DynamicFixture, baseline_indexless)->Arg(MAX_BENCH_SIZE);
BENCHMARK_REGISTER_F(DynamicFixture, signed_traditional_indexless)->Arg(MAX_BENCH_SIZE);
BENCHMARK_REGISTER_F(DynamicFixture, unsigned_traditional_indexless)->Arg(MAX_BENCH_SIZE);
BENCHMARK_REGISTER_F(DynamicFixture, alternative_indexless)->Arg(MAX_BENCH_SIZE);
BENCHMARK_REGISTER_F(DynamicFixture, range_indexless)->Arg(MAX_BENCH_SIZE);
BENCHMARK_REGISTER_F(DynamicFixture, power_indexless)->Arg(MAX_BENCH_SIZE);
BENCHMARK_REGISTER_F(DynamicFixture, eytzinger_hintless_indexless)->Arg(MAX_BENCH_SIZE);
BENCHMARK_REGISTER_F(DynamicFixture, eytzinger_branchless_indexless)->Arg(MAX_BENCH_SIZE);
BENCHMARK_REGISTER_F(DynamicFixture, eytzinger_prefetching_indexless)->Arg(MAX_BENCH_SIZE);

#define STATIC_FIXTURE_REGISTER_TEST1(NAME) BENCHMARK_REGISTER_F(STATIC_FIXTURE_NAME(NAME), power_indexless)
#define STATIC_FIXTURE_REGISTER_TEST2(NAME) BENCHMARK_REGISTER_F(STATIC_FIXTURE_NAME(NAME), eytzinger_hintless_indexless)
#define STATIC_FIXTURE_REGISTER_TEST3(NAME) BENCHMARK_REGISTER_F(STATIC_FIXTURE_NAME(NAME), eytzinger_branchless_indexless)
#define STATIC_FIXTURE_REGISTER_TEST4(NAME) BENCHMARK_REGISTER_F(STATIC_FIXTURE_NAME(NAME), eytzinger_prefetching_indexless)

STATIC_FIXTURE_REGISTER_TEST1()->Arg(MAX_BENCH_SIZE);
STATIC_FIXTURE_REGISTER_TEST2()->Arg(MAX_BENCH_SIZE);
STATIC_FIXTURE_REGISTER_TEST3()->Arg(MAX_BENCH_SIZE);
STATIC_FIXTURE_REGISTER_TEST4()->Arg(MAX_BENCH_SIZE);

#define STATIC_FIXTURE_REGISTER_TEST_NAME(N) STATIC_FIXTURE_REGISTER_TEST1(_##N)
APPLY_MACRO(STATIC_FIXTURE_REGISTER_TEST_NAME, ->Repetitions(REPEATIONS))
#undef STATIC_FIXTURE_REGISTER_TEST_NAME
#define STATIC_FIXTURE_REGISTER_TEST_NAME(N) STATIC_FIXTURE_REGISTER_TEST2(_##N)
APPLY_MACRO(STATIC_FIXTURE_REGISTER_TEST_NAME, ->Repetitions(REPEATIONS))
#undef STATIC_FIXTURE_REGISTER_TEST_NAME
#define STATIC_FIXTURE_REGISTER_TEST_NAME(N) STATIC_FIXTURE_REGISTER_TEST3(_##N)
APPLY_MACRO(STATIC_FIXTURE_REGISTER_TEST_NAME, ->Repetitions(REPEATIONS))
#undef STATIC_FIXTURE_REGISTER_TEST_NAME
#define STATIC_FIXTURE_REGISTER_TEST_NAME(N) STATIC_FIXTURE_REGISTER_TEST4(_##N)
APPLY_MACRO(STATIC_FIXTURE_REGISTER_TEST_NAME, ->Repetitions(REPEATIONS))

BENCHMARK_REGISTER_F(C_Fixture, search_first)->STATISTICS(REPEATIONS)->Arg(MAX_BENCH_SIZE);
BENCHMARK_REGISTER_F(C_Fixture, search_last)->STATISTICS(REPEATIONS)->Arg(MAX_BENCH_SIZE);
BENCHMARK_REGISTER_F(C_Fixture, fail_first)->STATISTICS(REPEATIONS)->Arg(MAX_BENCH_SIZE);
BENCHMARK_REGISTER_F(C_Fixture, fail_last)->STATISTICS(REPEATIONS)->Arg(MAX_BENCH_SIZE);
