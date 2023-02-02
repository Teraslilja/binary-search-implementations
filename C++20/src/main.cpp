#if RUN_UNIT_TESTS
#include <gtest/gtest.h>
#endif

#if RUN_BENCHMARKS
#include <benchmark/benchmark.h>
#endif

#include <cstdlib>

int main([[maybe_unused]] int argc, [[maybe_unused]] char** argv)
{
#if RUN_UNIT_TESTS
    ::testing::InitGoogleTest(&argc, argv);
    int const status = RUN_ALL_TESTS();
    if (status != 0) {
        std::exit(status);
    }
#endif

#if RUN_BENCHMARKS
    ::benchmark::Initialize(&argc, argv);
    if (::benchmark::ReportUnrecognizedArguments(argc, argv)) {
        std::exit(EXIT_FAILURE);
    }
    ::benchmark::RunSpecifiedBenchmarks();
    ::benchmark::Shutdown();
#endif

    std::exit(EXIT_SUCCESS);
}
