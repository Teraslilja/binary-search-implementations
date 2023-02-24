#pragma once

//
// Helper functions for binary search implementations
//

namespace Helpers {
static inline constexpr bool is_power_of_two(std::size_t const N) noexcept
{
    return !(N & (N - std::size_t(1)));
}

static inline constexpr std::optional<int> log2(std::size_t const N) noexcept
{
    return N ? std::make_optional(63 - __builtin_clzll(N)) : std::nullopt;
}

// Return the largest 2^m, where N > 2^m
static inline constexpr std::size_t previous_power_of_two(std::size_t const N) noexcept
{
    if (is_power_of_two(N)) {
        return N >> 1u;
    }
    return std::size_t(1) << log2(N).value();
}
} // namespace Helpers

//
// Dynamic and static implementations of binary searche
//

template <typename D, typename I = std::size_t>
requires std::is_unsigned_v<I> && std::is_integral_v<I>
struct power : public binary_search<D, I> {
public:
    using data_t = binary_search<D, I>::data_t;
    using index_t = binary_search<D, I>::index_t;

    // Dynamic implementation
protected:
    // Have one ternary operation within loop, no bound checking as array
    // size is 2^m  and array index is unsigned and mid calculated with OR
    // operation
    [[nodiscard]] inline index_t constexpr withoutBoundCheck(std::span<data_t const> const data,
        data_t const v) const noexcept
    {
        index_t low = index_t(0);
        index_t width = static_cast<index_t>(Helpers::previous_power_of_two(data.size()));
        while (width > index_t(0)) {
            index_t const mid = low | width;
            low = (data[mid] <= v) ? mid : low;
            width >>= 1u;
        }
        return low;
    }

    // Have one ternary operation within loop and array index is unsigned
    // and mid calculated with OR operation
    [[nodiscard]] inline index_t constexpr withBoundCheck(std::span<data_t const> const data,
        data_t const v) const noexcept
    {
        index_t low = index_t(0);
        index_t width = static_cast<index_t>(Helpers::previous_power_of_two(data.size()));
        while (width > index_t(0)) {
            index_t const mid = low | width;
            low = ((mid < data.size()) && (data[mid] <= v)) ? mid : low;
            width >>= 1u;
        }
        return low;
    }

    // Static implementation
protected:
    // Have one ternary operation within loop, no bound checking as array
    // size is 2^m  and array index is unsigned and mid calculated with OR
    // operation
    template <std::size_t N>
    [[nodiscard]] inline index_t constexpr withoutBoundCheck(data_t const data[], data_t const v) const noexcept
    {
        index_t low = index_t(0);
        index_t constexpr W = static_cast<index_t>(Helpers::previous_power_of_two(N));
        index_t constexpr P = Helpers::log2(W).value_or(index_t(0));
        for (index_t width = W, p = index_t(0); p <= P; width >>= 1u, ++p) {
            index_t const mid = low | width;
            low = (data[mid] <= v) ? mid : low;
        }
        return low;
    }

    // Have one ternary operation within loop and array index is unsigned
    // and mid calculated with OR operation
    template <std::size_t N>
    [[nodiscard]] inline index_t constexpr withBoundCheck(data_t const data[], data_t const v) const noexcept
    {
        index_t low = index_t(0);
        index_t constexpr W = static_cast<index_t>(Helpers::previous_power_of_two(N));
        index_t constexpr P = Helpers::log2(W).value_or(index_t(0));
        for (index_t width = W, p = index_t(0); p <= P; width >>= 1u, ++p) {
            index_t const mid = low | width;
            low = ((mid < N) && (data[mid] <= v)) ? mid : low;
        }
        return low;
    }

public:
    [[nodiscard]] std::optional<index_t> constexpr impl(std::span<data_t const> const data,
        data_t const v) const noexcept override
    {
        if (data.size() > 0u) {
            index_t const low = Helpers::is_power_of_two(data.size()) ? withoutBoundCheck(data, v) : withBoundCheck(data, v);
            return (data[low] == v) ? std::make_optional(low) : std::nullopt;
        }
        return std::nullopt;
    }

    template <size_t N>
    [[nodiscard]] std::optional<index_t> constexpr impl(std::array<data_t, N> const data, data_t const v) const noexcept
    {
        if (N > 0u) {
            index_t const low = Helpers::is_power_of_two(N) ? withoutBoundCheck<N>(data.data(), v) : withBoundCheck<N>(data.data(), v);
            return (data[low] == v) ? std::make_optional(low) : std::nullopt;
        }
        return std::nullopt;
    }
};
