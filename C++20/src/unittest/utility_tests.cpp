#include <gtest/gtest.h>

#include <iostream>

// import binary_search;
import binary_search.power;

//
// Tests for helper functionality
//

struct ExpectedResults {
  bool const is_power_of_2_or_zero;
  std::optional<int> const log2;
  std::size_t const previous_power;

  friend std::ostream &operator<<(std::ostream &out,
                                  ExpectedResults const &data) {
    out << "{";
    out << (data.is_power_of_2_or_zero ? "true" : "false") << ","
        << (data.log2.has_value() ? std::to_string(data.log2.value()) : "<nil>")
        << "," << data.previous_power;
    out << "}";
    return out;
  }
};

struct HelperData {
  std::size_t const value;
  ExpectedResults const expected_results;

  friend std::ostream &operator<<(std::ostream &out, HelperData const &data) {
    out << "{";
    out << data.value << "," << data.expected_results;
    out << "}";
    return out;
  }
};

class HelperTests : public ::testing::TestWithParam<HelperData> {};

TEST_P(HelperTests, ispower2OrZero_correctAnswer) {
  auto const params = GetParam();

  bool const result = Helper::is_power_of_two_or_zero(params.value);
  ASSERT_EQ(result, params.expected_results.is_power_of_2_or_zero);
}

TEST_P(HelperTests, log2_correctAnswer) {
  auto const params = GetParam();

  std::optional<int> const result = Helper::log2(params.value);
  ASSERT_EQ(result, params.expected_results.log2);
}

TEST_P(HelperTests, previousPower_correctAnswer) {
  auto const params = GetParam();

  std::size_t const result = Helper::previous_power_of_two(params.value);
  ASSERT_EQ(result, params.expected_results.previous_power);
}

static HelperData constexpr set1[] = {
    {UINT64_C(1) << 0u, {true, std::make_optional(0), UINT64_C(0)}},
    {UINT64_C(1) << 1u, {true, std::make_optional(1), UINT64_C(1) << 0u}},
    {UINT64_C(1) << 2u, {true, std::make_optional(2), UINT64_C(1) << 1u}},
    {UINT64_C(1) << 3u, {true, std::make_optional(3), UINT64_C(1) << 2u}},
    {UINT64_C(1) << 4u, {true, std::make_optional(4), UINT64_C(1) << 3u}},
    {UINT64_C(1) << 5u, {true, std::make_optional(5), UINT64_C(1) << 4u}},
    {UINT64_C(1) << 6u, {true, std::make_optional(6), UINT64_C(1) << 5u}},
    {UINT64_C(1) << 7u, {true, std::make_optional(7), UINT64_C(1) << 6u}},
};
INSTANTIATE_TEST_SUITE_P(ExactPowers, HelperTests, ::testing::ValuesIn(set1));

static HelperData constexpr set2[] = {
    {(UINT64_C(1) << 2u) - 1u,
     {false, std::make_optional(1), UINT64_C(1) << 1u}},
    {(UINT64_C(1) << 3u) - 1u,
     {false, std::make_optional(2), UINT64_C(1) << 2u}},
    {(UINT64_C(1) << 4u) - 1u,
     {false, std::make_optional(3), UINT64_C(1) << 3u}},
    {(UINT64_C(1) << 5u) - 1u,
     {false, std::make_optional(4), UINT64_C(1) << 4u}},
    {(UINT64_C(1) << 6u) - 1u,
     {false, std::make_optional(5), UINT64_C(1) << 5u}},
    {(UINT64_C(1) << 7u) - 1u,
     {false, std::make_optional(6), UINT64_C(1) << 6u}},
};
INSTANTIATE_TEST_SUITE_P(UnderPowers, HelperTests, ::testing::ValuesIn(set2));

static HelperData constexpr set3[] = {
    {(UINT64_C(1) << 1u) + 1u,
     {false, std::make_optional(1), UINT64_C(1) << 1u}},
    {(UINT64_C(1) << 2u) + 1u,
     {false, std::make_optional(2), UINT64_C(1) << 2u}},
    {(UINT64_C(1) << 3u) + 1u,
     {false, std::make_optional(3), UINT64_C(1) << 3u}},
    {(UINT64_C(1) << 4u) + 1u,
     {false, std::make_optional(4), UINT64_C(1) << 4u}},
    {(UINT64_C(1) << 5u) + 1u,
     {false, std::make_optional(5), UINT64_C(1) << 5u}},
    {(UINT64_C(1) << 6u) + 1u,
     {false, std::make_optional(6), UINT64_C(1) << 6u}},
    {(UINT64_C(1) << 7u) + 1u,
     {false, std::make_optional(7), UINT64_C(1) << 7u}},
};
INSTANTIATE_TEST_SUITE_P(OverPowers, HelperTests, ::testing::ValuesIn(set3));

static HelperData constexpr set4[] = {
    {UINT64_C(0), {true, std::nullopt, UINT64_C(0)}},
    {UINT64_C(1), {true, std::make_optional(0), UINT64_C(0)}},
    {UINT64_C(1) << 61u, {true, std::make_optional(61), UINT64_C(1) << 60u}},
    {UINT64_C(1) << 62u, {true, std::make_optional(62), UINT64_C(1) << 61u}},
    {UINT64_C(1) << 63u, {true, std::make_optional(63), UINT64_C(1) << 62u}},
    {std::numeric_limits<std::uint8_t>::max(),
     {false, std::make_optional(7), UINT64_C(1) << 7u}},
    {std::numeric_limits<std::uint16_t>::max(),
     {false, std::make_optional(15), UINT64_C(1) << 15u}},
    {std::numeric_limits<std::uint32_t>::max(),
     {false, std::make_optional(31), UINT64_C(1) << 31u}},
    {std::numeric_limits<std::uint64_t>::max(),
     {false, std::make_optional(63), UINT64_C(1) << 63u}},
    {std::numeric_limits<std::size_t>::max(),
     {false, std::make_optional(63), UINT64_C(1) << 63u}},
};
INSTANTIATE_TEST_SUITE_P(Specials, HelperTests, ::testing::ValuesIn(set4));
