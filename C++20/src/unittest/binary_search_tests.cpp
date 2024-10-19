#include <gtest/gtest-spi.h>
#include <gtest/gtest.h>

#include "binary_search.hpp"
#include "helper.hpp"

//
// Configuration
//

using DataType = std::int32_t;

//
// Tests with index type of size of int or larger
//

using IndexType = std::size_t;

class BinarySearchDynamicTests : public ::testing::TestWithParam<std::size_t> {
protected:
    void SetUp() override
    {
        testdata_ = filler<DataType, IndexType>(GetParam());
    }

    void TearDown() override
    {
    }

    std::vector<DataType> testdata_;
};

TEST_P(BinarySearchDynamicTests, signed_traditional)
{
    ASSERT_TRUE(test(signed_traditional<DataType> {}, testdata_));
}

TEST_P(BinarySearchDynamicTests, unsigned_traditional)
{
    ASSERT_TRUE(test(unsigned_traditional<DataType, IndexType> {}, testdata_));
}

TEST_P(BinarySearchDynamicTests, alternative)
{
    ASSERT_TRUE(test(alternative<DataType, IndexType> {}, testdata_));
}

TEST_P(BinarySearchDynamicTests, range)
{
    ASSERT_TRUE(test(range<DataType, IndexType> {}, testdata_));
}

TEST_P(BinarySearchDynamicTests, power)
{
    ASSERT_TRUE(test(power<DataType, IndexType> {}, testdata_));
}

TEST_P(BinarySearchDynamicTests, eytzinger_hintless)
{
    std::vector<DataType> eytzinger_layout(testdata_.size());
    EXPECT_TRUE(Helpers::eytzinger_layout(std::span(eytzinger_layout.begin(), eytzinger_layout.size()),
        std::span(testdata_.cbegin(), testdata_.size())));

    ASSERT_TRUE(test(eytzinger_hintless<DataType, IndexType> {}, eytzinger_layout));
}

TEST_P(BinarySearchDynamicTests, eytzinger_branchless)
{
    std::vector<DataType> eytzinger_layout(testdata_.size());
    EXPECT_TRUE(Helpers::eytzinger_layout(std::span(eytzinger_layout.begin(), eytzinger_layout.size()),
        std::span(testdata_.cbegin(), testdata_.size())));

    ASSERT_TRUE(test(eytzinger_branchless<DataType, IndexType> {}, eytzinger_layout));
}

TEST_P(BinarySearchDynamicTests, eytzinger_prefetching)
{
    std::vector<DataType> eytzinger_layout(testdata_.size());
    EXPECT_TRUE(Helpers::eytzinger_layout(std::span(eytzinger_layout.begin(), eytzinger_layout.size()),
        std::span(testdata_.cbegin(), testdata_.size())));

    ASSERT_TRUE(test(eytzinger_prefetching<DataType, IndexType> {}, eytzinger_layout));
}

static std::size_t constexpr set5[] = { 0u, 1u, 2u, 3u, 10u, 100u, 511u, 512u, 513u, 1u << 15u };
INSTANTIATE_TEST_SUITE_P(ArraySizes, BinarySearchDynamicTests, ::testing::ValuesIn(set5));

// Static tests of power
class BinarySearchStaticTests : public ::testing::Test {
protected:
    void SetUp() override
    {
    }

    void TearDown() override
    {
    }

    template <std::size_t N>
    bool constexpr test()
    {
        std::array<DataType, N> constexpr testdata2_ = filler2<DataType, IndexType, N>();

        return ::test(power<DataType> {}, testdata2_);
    }
};

TEST_F(BinarySearchStaticTests, size_0)
{
    bool const result = test<0u>();
    ASSERT_TRUE(result);
}

TEST_F(BinarySearchStaticTests, size_1)
{
    bool const result = test<1u>();
    ASSERT_TRUE(result);
}

TEST_F(BinarySearchStaticTests, size_2)
{
    bool const result = test<2u>();
    ASSERT_TRUE(result);
}

TEST_F(BinarySearchStaticTests, size_3)
{
    bool const result = test<3u>();
    ASSERT_TRUE(result);
}

TEST_F(BinarySearchStaticTests, size_10)
{
    bool const result = test<10u>();
    ASSERT_TRUE(result);
}

TEST_F(BinarySearchStaticTests, size_100)
{
    bool const result = test<100u>();
    ASSERT_TRUE(result);
}

TEST_F(BinarySearchStaticTests, size_511)
{
    bool const result = test<511u>();
    ASSERT_TRUE(result);
}

TEST_F(BinarySearchStaticTests, size_512)
{
    bool const result = test<512u>();
    ASSERT_TRUE(result);
}

TEST_F(BinarySearchStaticTests, size_513)
{
    bool const result = test<513u>();
    ASSERT_TRUE(result);
}

TEST_F(BinarySearchStaticTests, size_64ki)
{
    bool const result = test<1u << 16u>();
    ASSERT_TRUE(result);
}

//
// Tests with index sizes smaller than int
//

using SmallIndexType = std::uint8_t;

class TimeoutDeathTests : public ::testing::Test {
protected:
    void SetUp() override
    {
        timeout_timer_ = nullptr;
    }

    void TearDown() override
    {
        if (timeout_timer_ != nullptr) {
            ASSERT_EQ(0, timer_delete(timeout_timer_));
        }
    }

    inline void killThreadAfterTimeout()
    {
        createTimeoutTimer();
        startTimeoutTimer();
    }

private:
    void createTimeoutTimer()
    {
        // Send kill signal after timeout
        struct sigevent kill_signal_after_timeout {
            .sigev_value = { .sival_ptr = &timeout_timer_ }, .sigev_signo = SIGKILL, .sigev_notify = SIGEV_SIGNAL,
            ._sigev_un = {
                0
            }
        };
        ASSERT_EQ(0, timer_create(CLOCK_REALTIME, &kill_signal_after_timeout, &timeout_timer_));
    }

    void startTimeoutTimer()
    {
        time_t constexpr DURATION_S = 0;
        long constexpr DURATION_NS = 100'000'000; // 1 ms = 1000 us = 10^6 ns
        itimerspec constexpr timeout_duration { .it_interval = { 0, 0 }, .it_value = { DURATION_S, DURATION_NS } };

        ASSERT_EQ(0, timer_settime(timeout_timer_, 0, &timeout_duration, nullptr));
    }

    timer_t timeout_timer_;
};

TEST_F(TimeoutDeathTests, signed_traditionalShallTimeout)
{
    using SmallIndexType = std::int8_t;
    enum : std::size_t {
        SIZE = static_cast<std::size_t>(std::numeric_limits<SmallIndexType>::max()) + 1u
    };
    static_assert(SIZE == 128u, "");
    std::vector<DataType> const data = filler<DataType, SmallIndexType>(SIZE);

    ASSERT_DEATH(
        {
            killThreadAfterTimeout();

            bool const result = test(signed_traditional<DataType, SmallIndexType> {}, data);
            ASSERT_TRUE(result);
        },
        "");
}

TEST_F(TimeoutDeathTests, unsigned_traditionalShallTimeout)
{
    enum : std::size_t {
        SIZE = std::numeric_limits<SmallIndexType>::max() + 1u
    };
    static_assert(SIZE == 256u);
    std::vector<DataType> const data = filler<DataType, SmallIndexType>(SIZE);

    ASSERT_DEATH(
        {
            killThreadAfterTimeout();

            bool const result = test(unsigned_traditional<DataType, SmallIndexType> {}, data);
            ASSERT_TRUE(result);
        },
        "");
}

TEST_F(TimeoutDeathTests, alternativeShallTimeout)
{
    enum : std::size_t {
        SIZE = std::numeric_limits<SmallIndexType>::max() + 1u
    };
    static_assert(SIZE == 256u);
    std::vector<DataType> const data = filler<DataType, SmallIndexType>(SIZE);

    ASSERT_DEATH(
        {
            killThreadAfterTimeout();

            bool const result = test(alternative<DataType, SmallIndexType> {}, data);
            ASSERT_TRUE(result);
        },
        "");
}

class ProveAsIncorrectTests : public ::testing::Test {
};

TEST_F(ProveAsIncorrectTests, rangeFails)
{
    enum : std::size_t {
        SIZE = std::numeric_limits<SmallIndexType>::max() + 1u
    };
    static_assert(SIZE == 256u);
    static std::vector<DataType> data = filler<DataType, SmallIndexType>(SIZE);

    EXPECT_FATAL_FAILURE(
        {
            bool const result = test(range<DataType, SmallIndexType> {}, data);
            ASSERT_TRUE(result);
        },
        "");
}

class ProveAsCorrectTests : public ::testing::Test {
};

TEST_F(ProveAsCorrectTests, dynamic_powerPasses)
{
    enum : std::size_t {
        SIZE = std::numeric_limits<SmallIndexType>::max() + 1u
    };
    static_assert(SIZE == 256u, "");
    std::vector<DataType> const data = filler<DataType, SmallIndexType>(SIZE);

    bool const result = test(power<DataType, SmallIndexType> {}, data);
    ASSERT_TRUE(result);
}

TEST_F(ProveAsCorrectTests, static_powerPasses)
{
    enum : std::size_t {
        SIZE = std::numeric_limits<SmallIndexType>::max() + 1u
    };
    static_assert(SIZE == 256u, "");
    std::array<DataType, SIZE> const data = filler2<DataType, SmallIndexType, SIZE>();

    bool const result = test(power<DataType, SmallIndexType> {}, data);
    ASSERT_TRUE(result);
}
