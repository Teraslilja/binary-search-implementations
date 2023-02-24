#pragma once

//
// Dynamic and static implementations of binary searche
//

template <typename D, typename I = std::size_t>
requires std::is_unsigned_v<I> && std::is_integral_v<I>
struct traditional2 : public binary_search<D, I> {
public:
    using data_t = binary_search<D, I>::data_t;
    using index_t = binary_search<D, I>::index_t;

private:
    // Have two conditions within loop and array index is unsigned
    [[nodiscard]] inline std::optional<index_t> constexpr twoConditions(std::span<data_t const> const data,
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
            } else if (data[mid] < v) {
                low = mid + index_t(1);
            } else {
                return std::make_optional(mid);
            }
        }
        return (low < data.size()) && (data[low] == v) ? std::make_optional(low) : std::nullopt;
    }

public:
    [[nodiscard]] std::optional<index_t> constexpr impl(std::span<data_t const> const data,
        data_t const v) const noexcept override
    {
        return (data.size() > I(0)) ? twoConditions(data, v) : std::nullopt;
    }

    template <std::size_t N>
    [[nodiscard]] inline std::optional<index_t> constexpr impl(std::array<data_t const, N> const data,
        data_t const v) const noexcept
    {
        return impl(std::span(data), v);
    }
};
