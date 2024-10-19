#include <gmock/gmock-matchers.h>
#include <gtest/gtest.h>

#include <iostream>

#include "binary_search.hpp"

//
// Tests for helper functionality
//

struct ExpectedResults {
    bool const is_power_of_2_or_zero;
    std::optional<std::size_t> const log2;
    std::size_t const previous_power;

    friend std::ostream& operator<<(std::ostream& out, ExpectedResults const& data)
    {
        out << "{";
        out << (data.is_power_of_2_or_zero ? "true" : "false") << "," << (data.log2.has_value() ? std::to_string(data.log2.value()) : "<nil>")
            << "," << data.previous_power;
        out << "}";
        return out;
    }
};

struct HelperData {
    std::size_t const value;
    ExpectedResults const expected_results;

    friend std::ostream& operator<<(std::ostream& out, HelperData const& data)
    {
        out << "{";
        out << data.value << "," << data.expected_results;
        out << "}";
        return out;
    }
};

class HelperTests : public ::testing::TestWithParam<HelperData> {
};

TEST_P(HelperTests, ispower2OrZero_correctAnswer)
{
    auto const params = GetParam();

    bool const result = Helpers::is_power_of_two_or_zero(params.value);
    ASSERT_EQ(result, params.expected_results.is_power_of_2_or_zero);
}

TEST_P(HelperTests, log2_correctAnswer)
{
    auto const params = GetParam();

    std::optional<std::size_t> const result = Helpers::log2(params.value);
    ASSERT_EQ(result, params.expected_results.log2);
}

TEST_P(HelperTests, previousPower_correctAnswer)
{
    auto const params = GetParam();

    std::size_t const result = Helpers::previous_power_of_two(params.value);
    ASSERT_EQ(result, params.expected_results.previous_power);
}

static HelperData constexpr set1[] = {
    { UINT64_C(1) << 0u, { true, std::make_optional(0u), UINT64_C(0) } },
    { UINT64_C(1) << 1u, { true, std::make_optional(1u), UINT64_C(1) << 0u } },
    { UINT64_C(1) << 2u, { true, std::make_optional(2u), UINT64_C(1) << 1u } },
    { UINT64_C(1) << 3u, { true, std::make_optional(3u), UINT64_C(1) << 2u } },
    { UINT64_C(1) << 4u, { true, std::make_optional(4u), UINT64_C(1) << 3u } },
    { UINT64_C(1) << 5u, { true, std::make_optional(5u), UINT64_C(1) << 4u } },
    { UINT64_C(1) << 6u, { true, std::make_optional(6u), UINT64_C(1) << 5u } },
    { UINT64_C(1) << 7u, { true, std::make_optional(7u), UINT64_C(1) << 6u } },
};
INSTANTIATE_TEST_SUITE_P(ExactPowers, HelperTests, ::testing::ValuesIn(set1));

static HelperData constexpr set2[] = {
    { (UINT64_C(1) << 2u) - 1u, { false, std::make_optional(1u), UINT64_C(1) << 1u } },
    { (UINT64_C(1) << 3u) - 1u, { false, std::make_optional(2u), UINT64_C(1) << 2u } },
    { (UINT64_C(1) << 4u) - 1u, { false, std::make_optional(3u), UINT64_C(1) << 3u } },
    { (UINT64_C(1) << 5u) - 1u, { false, std::make_optional(4u), UINT64_C(1) << 4u } },
    { (UINT64_C(1) << 6u) - 1u, { false, std::make_optional(5u), UINT64_C(1) << 5u } },
    { (UINT64_C(1) << 7u) - 1u, { false, std::make_optional(6u), UINT64_C(1) << 6u } },
};
INSTANTIATE_TEST_SUITE_P(UnderPowers, HelperTests, ::testing::ValuesIn(set2));

static HelperData constexpr set3[] = {
    { (UINT64_C(1) << 1u) + 1u, { false, std::make_optional(1u), UINT64_C(1) << 1u } },
    { (UINT64_C(1) << 2u) + 1u, { false, std::make_optional(2u), UINT64_C(1) << 2u } },
    { (UINT64_C(1) << 3u) + 1u, { false, std::make_optional(3u), UINT64_C(1) << 3u } },
    { (UINT64_C(1) << 4u) + 1u, { false, std::make_optional(4u), UINT64_C(1) << 4u } },
    { (UINT64_C(1) << 5u) + 1u, { false, std::make_optional(5u), UINT64_C(1) << 5u } },
    { (UINT64_C(1) << 6u) + 1u, { false, std::make_optional(6u), UINT64_C(1) << 6u } },
    { (UINT64_C(1) << 7u) + 1u, { false, std::make_optional(7u), UINT64_C(1) << 7u } },
};
INSTANTIATE_TEST_SUITE_P(OverPowers, HelperTests, ::testing::ValuesIn(set3));

static HelperData constexpr set4[] = {
    { UINT64_C(0), { true, std::nullopt, UINT64_C(0) } },
    { UINT64_C(1), { true, std::make_optional(0u), UINT64_C(0) } },
    { UINT64_C(1) << 61u, { true, std::make_optional(61u), UINT64_C(1) << 60u } },
    { UINT64_C(1) << 62u, { true, std::make_optional(62u), UINT64_C(1) << 61u } },
    { UINT64_C(1) << 63u, { true, std::make_optional(63u), UINT64_C(1) << 62u } },
    { std::numeric_limits<std::uint8_t>::max(), { false, std::make_optional(7u), UINT64_C(1) << 7u } },
    { std::numeric_limits<std::uint16_t>::max(), { false, std::make_optional(15u), UINT64_C(1) << 15u } },
    { std::numeric_limits<std::uint32_t>::max(), { false, std::make_optional(31u), UINT64_C(1) << 31u } },
    { std::numeric_limits<std::uint64_t>::max(), { false, std::make_optional(63u), UINT64_C(1) << 63u } },
    { std::numeric_limits<std::size_t>::max(), { false, std::make_optional(63u), UINT64_C(1) << 63u } },
};
INSTANTIATE_TEST_SUITE_P(Specials, HelperTests, ::testing::ValuesIn(set4));

struct SpecialEyzingerData {
    std::vector<int> const monotonic;
    std::size_t const reserved_space;

    friend std::ostream& operator<<(std::ostream& out, SpecialEyzingerData const& data)
    {
        auto const helper = [&out](std::vector<int> const& v) -> void {
            if (!v.empty()) {
                out << "{ ";
                for (int const i : v) {
                    out << i << " ";
                }
                out << "}";
            } else {
                out << "{}";
            }
        };

        out << "{";
        helper(data.monotonic);
        out << ", ";
        out << data.reserved_space;
        out << "}";
        return out;
    }
};

class SpecialEyzingerTests : public ::testing::TestWithParam<SpecialEyzingerData> {
};

TEST_P(SpecialEyzingerTests, eytzingerlayout_falseAsReturnValue)
{
    auto const params = GetParam();

    std::vector<int> layout(params.reserved_space);
    bool const result = eytzinger_base<int>::eytzinger_layout(std::span(layout), std::span(params.monotonic));
    ASSERT_FALSE(result);
}

static SpecialEyzingerData const special_eyzinger[] = {
    { {}, 1u },
};
INSTANTIATE_TEST_SUITE_P(Specials, SpecialEyzingerTests, ::testing::ValuesIn(special_eyzinger));

struct NormalEyzingerData {
    std::vector<int> const monotonic;
    std::vector<int> const expected_result;

    friend std::ostream& operator<<(std::ostream& out, NormalEyzingerData const& data)
    {
        auto const helper = [&out](std::vector<int> const& v) -> void {
            if (!v.empty()) {
                out << "{ ";
                for (int const i : v) {
                    out << i << " ";
                }
                out << "}";
            } else {
                out << "{}";
            }
        };

        out << "{";
        helper(data.monotonic);
        out << ", ";
        helper(data.expected_result);
        out << "}";
        return out;
    }
};

class NormalEyzingerTests : public ::testing::TestWithParam<NormalEyzingerData> {
};

TEST_P(NormalEyzingerTests, eytzingerlayout_correctAnswer)
{
    auto const params = GetParam();
    EXPECT_EQ(params.expected_result.size(), params.monotonic.size());

    std::vector<int> layout(params.monotonic.size());
    bool const result = eytzinger_base<int>::eytzinger_layout(std::span(layout), std::span(params.monotonic));

    ASSERT_TRUE(result);
    ASSERT_THAT(layout, ::testing::Eq(params.expected_result));
}

static NormalEyzingerData const normals_eyzinger[] = {
    { {}, {} },
    { { 1 }, { 1 } },
    { { 1, 2 }, { 2, 1 } },
    { { 1, 2, 3 }, { 2, 1, 3 } },
    { { 1, 2, 3, 4, 5, 6, 7 }, { 4, 2, 6, 1, 3, 5, 7 } },
    { { 1, 2, 3, 4, 5, 6, 7, 8 }, { 5, 3, 7, 2, 4, 6, 8, 1 } },
    { { 1, 2, 3, 4, 5, 6, 7, 8, 9 }, { 6, 4, 8, 2, 5, 7, 9, 1, 3 } },
    { { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 }, { 7, 4, 9, 2, 6, 8, 10, 1, 3, 5 } },
    { { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 }, { 8, 4, 10, 2, 6, 9, 11, 1, 3, 5, 7 } },
    { { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12 }, { 8, 4, 11, 2, 6, 10, 12, 1, 3, 5, 7, 9 } },
    { { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13 }, { 8, 4, 12, 2, 6, 10, 13, 1, 3, 5, 7, 9, 11 } },
    { { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14 }, { 8, 4, 12, 2, 6, 10, 14, 1, 3, 5, 7, 9, 11, 13 } },
    { { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 }, { 8, 4, 12, 2, 6, 10, 14, 1, 3, 5, 7, 9, 11, 13, 15 } },
};
INSTANTIATE_TEST_SUITE_P(Normals, NormalEyzingerTests, ::testing::ValuesIn(normals_eyzinger));
