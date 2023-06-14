module;
#include <array>
#include <cassert>
#include <limits>
#include <vector>

//
// Helper functions for unit tests and benchmarks
//
export module helper;
import binary_search;

export {
  template <typename D, typename I>
  auto const filler = [](std::size_t const N) {
    [[maybe_unused]] D constexpr maxValue = std::numeric_limits<D>::max();
    [[maybe_unused]] std::size_t constexpr maxCount =
        std::min(static_cast<std::size_t>(std::numeric_limits<I>::max()),
                 std::numeric_limits<std::size_t>::max() - 1u) -
        0u + 1u;
    [[maybe_unused]] static D const maxDoubleValue =
        static_cast<D>((N - 1u) << 1u);
    assert(N <= maxCount);
    assert(maxDoubleValue <= maxValue);
    std::vector<D> tmp;
    tmp.reserve(N);
    for (std::size_t i = 0u; i < N; ++i) {
      D const dv = static_cast<D>(i) << 1u;
      assert((i << 1u) == static_cast<std::size_t>(dv));
      tmp.emplace_back(dv);
    }
    assert(tmp.size() == N);
    return tmp;
  };

  template <typename D, typename I, std::size_t N>
  auto constexpr filler2 = []() {
    D constexpr maxValue = std::numeric_limits<D>::max();
    std::size_t constexpr maxCount =
        std::min(static_cast<std::size_t>(std::numeric_limits<I>::max()),
                 std::numeric_limits<std::size_t>::max() - 1u) -
        0u + 1u;
    D constexpr maxDoubleValue = static_cast<D>((N - 1u) << 1u);
    static_assert(N <= maxCount);
    static_assert(maxDoubleValue <= maxValue);
    std::array<D, N> tmp;
    for (std::size_t i = 0u; i < N; ++i) {
      D const dv = static_cast<D>(i) << 1u;
      assert((i << 1u) == static_cast<std::size_t>(dv));
      tmp.at(i) = dv;
    }
    return tmp;
  };

  // Validate also indices
  template <typename D, typename I>
  bool constexpr test(binary_search<D, I> const &bs,
                      std::vector<D> const &data) noexcept {
    using data_t = D;

    data_t const startValue =
        (data.size() > 0u) ? (data.front() - data_t(1)) : data_t(-1);
    data_t const endValue =
        (data.size() > 0u) ? (data.back() + data_t(1)) : data_t(1);
    bool success = true;
    for (data_t value = startValue; value <= endValue; ++value) {
      auto const result = bs.impl(data, value);
      bool const subsuccess =
          ((value % 2 == 0) && (data.size() > 0u))
              ? (result.has_value() &&
                 (value == data.at(static_cast<std::size_t>(result.value()))))
              : (!result.has_value());
      success = success && subsuccess;
    }
    return success;
  }

  template <typename D, std::size_t N, typename I>
  bool constexpr test(binary_search<D, I> const &bs,
                      std::array<D, N> const &data) noexcept {
    using data_t = D;

    data_t const startValue =
        (data.size() > 0u) ? (data.front() - data_t(1)) : data_t(-1);
    data_t const endValue =
        (data.size() > 0u) ? (data.back() + data_t(1)) : data_t(1);
    bool success = true;
    for (data_t value = startValue; value <= endValue; ++value) {
      auto const result = bs.impl(data, value);
      bool const subsuccess =
          ((value % 2 == 0) && (N > 0u))
              ? (result.has_value() &&
                 (value == data.at(static_cast<std::size_t>(result.value()))))
              : (!result.has_value());
      success = success && subsuccess;
    }
    return success;
  }

  // Do not verify indices
  template <typename D, typename I>
  bool constexpr test_indexless(binary_search<D, I> const &bs,
                                std::vector<D> const &data) noexcept {
    using data_t = D;

    data_t const startValue =
        (data.size() > 0u) ? (data.front() - data_t(1)) : data_t(-1);
    data_t const endValue =
        (data.size() > 0u) ? (data.back() + data_t(1)) : data_t(1);
    bool success = true;
    for (data_t value = startValue; value <= endValue; ++value) {
      auto const result = bs.impl(data, value);
      bool const subsuccess = ((value % 2 == 0) && (data.size() > 0u))
                                  ? (result.has_value())
                                  : (!result.has_value());
      success = success && subsuccess;
    }
    return success;
  }

  template <typename D, std::size_t N, typename I>
  bool constexpr test_indexless(binary_search<D, I> const &bs,
                                std::array<D, N> const &data) noexcept {
    using data_t = D;

    data_t const startValue =
        (data.size() > 0u) ? (data.front() - data_t(1)) : data_t(-1);
    data_t const endValue =
        (data.size() > 0u) ? (data.back() + data_t(1)) : data_t(1);
    bool success = true;
    for (data_t value = startValue; value <= endValue; ++value) {
        auto const result = bs.impl(data, value);
        bool const subsuccess = ((value % 2 == 0) && (N > 0u))
            ? (result.has_value())
            : (!result.has_value());
        success = success && subsuccess;
    }
    return success;
  }
}
