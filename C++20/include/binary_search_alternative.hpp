#pragma once

//
// Dynamic and static implementations of binary search
//

template <typename D, typename I = std::size_t>
requires std::is_unsigned_v<I> && std::is_integral_v<I>
struct alternative : public binary_search<D, I> {
public:
    using data_t = binary_search<D, I>::data_t;
    using index_t = binary_search<D, I>::index_t;

private:
    // Have single condition within loop and array index is unsigned
    [[nodiscard]] inline std::optional<index_t> constexpr singleCondition(std::span<data_t const> const data,
        data_t const v) const noexcept
    {
        index_t low = index_t(0);
        index_t high = static_cast<index_t>(data.size() - 1u);
        while (low < high) {
            // Actually broken: consider I=uint16_t, and data.size()==2^16 &
            // high=2^16-1,low=0 -> mid = 0 >> 1
            index_t const mid = low + (static_cast<index_t>((high - low) + index_t(1)) >> 1u);
            if (data[mid] > v) {
                high = mid - index_t(1);
            } else {
                low = mid;
            }
        }
        return (data[low] == v) ? std::make_optional(low) : std::nullopt;
    }

public:
    [[nodiscard]] std::optional<index_t> constexpr impl(std::span<data_t const> const data,
        data_t const v) const noexcept override
    {
        return (data.size() > 0u) ? singleCondition(data, v) : std::nullopt;
    }

    template <std::size_t N>
    [[nodiscard]] inline std::optional<index_t> constexpr impl(std::span<data_t const, N> const data,
        data_t const v) const noexcept
    {
        return impl(data, v);
    }
};
