module;
#include <array>
#include <cstddef>
#include <optional>
#include <span>
#include <type_traits>

export module binary_search.signed_traditional;
import binary_search;

//
// Dynamic and static implementations of binary search by traditional algorithm
//

export {
  template <typename D, typename I = std::ptrdiff_t>
  requires std::is_signed_v<I> && std::is_integral_v<I>
  struct signed_traditional : public binary_search<D, I> {
  public:
    using data_t = binary_search<D, I>::data_t;
    using index_t = binary_search<D, I>::index_t;

  private:
    // Have two conditions within loop and array index is signed
    [[nodiscard]] inline std::optional<index_t> constexpr twoConditions(
        std::span<data_t const> const data, data_t const v) const noexcept {
      index_t low = index_t(0);
      index_t high = static_cast<index_t>(data.size()) - index_t(1);
      while (low <= high) {
        index_t const mid = static_cast<index_t>(low + ((high - low) >> 1u));
        if (data[static_cast<std::size_t>(mid)] > v) {
          high = mid - index_t(1);
        } else if (data[static_cast<std::size_t>(mid)] < v) {
          low = mid + index_t(1);
        } else {
          return std::make_optional(mid);
        }
      }
      return std::nullopt;
    }

  public:
    [[nodiscard]] std::optional<index_t> constexpr impl(
        std::span<data_t const> const data, data_t const v) const noexcept {
      return (data.size() > index_t(0)) ? twoConditions(data, v) : std::nullopt;
    }

    template <std::size_t N>
    [[nodiscard]] inline std::optional<index_t> constexpr impl(
        std::array<data_t const, N> const data, data_t const v) const noexcept {
      return impl(std::span(data), v);
    }
  };
}
