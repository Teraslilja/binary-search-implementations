#pragma once

//
// Dynamic and static implementations of binary search
//

template <typename D, typename I = std::size_t>
requires std::is_unsigned_v<I> && std::is_integral_v<I>
struct range : public binary_search<D, I> {
public:
    using data_t = binary_search<D, I>::data_t;
    using index_t = binary_search<D, I>::index_t;

private:
    // Have one ternary operation within loop and array index is unsigned
    [[nodiscard]] inline std::optional<index_t> constexpr withWidth(std::span<data_t const> const data,
        data_t const v) const noexcept
    {
        index_t low = index_t(0);
        // Actually broken, if I=uint16_t and data.size()>=2^16-1
        index_t width = static_cast<index_t>(data.size());
        while (width > index_t(1)) {
            // Actually broken, if I=uint16_t and data.size()>=2^16-1
            width = static_cast<index_t>(width + index_t(1)) >> 1u;
            index_t const mid = low + width;
            low = ((mid < data.size()) && (data[mid] <= v)) ? mid : low;
        }
        return (data[low] == v) ? std::make_optional(low) : std::nullopt;
    }

public:
    [[nodiscard]] std::optional<index_t> constexpr impl(std::span<data_t const> const data,
        data_t const v) const noexcept override
    {
        return (data.size() > 0u) ? withWidth(data, v) : std::nullopt;
    }

    template <std::size_t N>
    [[nodiscard]] inline std::optional<index_t> constexpr impl(std::array<data_t const, N> const data,
        data_t const v) const noexcept
    {
        return impl(std::span(data), v);
    }
};
