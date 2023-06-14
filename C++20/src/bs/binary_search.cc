//
// Dynamic and static interfaces for binary search implementations
//
module;
#include <array>
#include <optional>
#include <span>

export module binary_search;

export {
  template <typename D, typename I> struct binary_search {
  public:
    using data_t = D;
    using index_t = I;

  public:
    [[nodiscard]] virtual std::optional<index_t> constexpr impl(
        std::span<data_t const> const data, data_t const v) const noexcept = 0;

    template <std::size_t N>
    [[nodiscard]] inline std::optional<index_t> constexpr impl(
        std::array<data_t const, N> const data, data_t const v) const noexcept;
  };
}
