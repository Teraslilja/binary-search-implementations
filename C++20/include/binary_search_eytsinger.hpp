#pragma once

#include <bit>
#include <iostream>
#include <span>

//
// Helper functions for binary search implementations
//

namespace Helpers {
template <typename T>
[[nodiscard]] static constexpr std::size_t eytzinger_layout(std::span<T> eytzinger, std::span<T const> const monotonic, std::size_t i, std::size_t const k) noexcept
{
    if ((k < eytzinger.size()) && (i < monotonic.size())) {
        std::size_t const k2 = (k + 1u) << 1u;
        i = eytzinger_layout(eytzinger, monotonic, i, k2 - 1u);
        eytzinger[k] = monotonic[i++];
        i = eytzinger_layout(eytzinger, monotonic, i, k2 - 0u);
    }
    return i;
}

template <typename T>
[[nodiscard]] constexpr bool eytzinger_layout(std::span<T> eytzinger, std::span<T const> const monotonic) noexcept
{
    if (eytzinger.size() != monotonic.size()) {
        return false;
    }

    std::size_t const len = eytzinger_layout<T>(eytzinger, monotonic, 0u, 0u);

    return len == monotonic.size();
}
} // namespace Helpers

//
// Eytzinger layout based implementations of binary searches
//

template <typename D, typename I = std::size_t>
    requires std::is_unsigned_v<I> && std::is_integral_v<I>
struct eytzinger_hintless : public binary_search<D, I> {
public:
    using data_t = binary_search<D, I>::data_t;
    using index_t = binary_search<D, I>::index_t;

private:
    [[nodiscard]] inline std::optional<index_t> constexpr hintless(std::span<data_t const> const data,
        data_t const v) const noexcept
    {
        index_t index = index_t(1);
        index_t const N = data.size();
        while (index <= N) {
            if (data[index - index_t(1)] < v) {
                index <<= 1u;
                index |= 1u;
            } else {
                index <<= 1u;
            }
        }
        index >>= index_t(__builtin_ffsll(static_cast<long long int>(~index)));
        return (index == index_t(0)) ? std::nullopt
                                     : ((data[index - index_t(1)] == v) ? std::make_optional(index - index_t(1))
                                                                        : std::nullopt);
    }

public:
    [[nodiscard]] std::optional<index_t> constexpr impl(std::span<data_t const> const data,
        data_t const v) const noexcept override
    {
        return (data.size() > index_t(0)) ? hintless(data, v) : std::nullopt;
    }

    template <size_t N>
    [[nodiscard]] std::optional<index_t> constexpr impl(std::span<data_t, N> const data, data_t const v) const noexcept
    {
        return impl(data, v);
    }
};

template <typename D, typename I = std::size_t>
    requires std::is_unsigned_v<I> && std::is_integral_v<I>
struct eytzinger_branchless : public binary_search<D, I> {
public:
    using data_t = binary_search<D, I>::data_t;
    using index_t = binary_search<D, I>::index_t;

private:
    [[nodiscard]] inline std::optional<index_t> constexpr branchless(std::span<data_t const> const data,
        data_t const v) const noexcept
    {
        index_t index = index_t(1);
        index_t const N = data.size();
        while (index <= N) {
            index = (index << 1u) | (data[index - index_t(1)] < v);
        }
        index >>= index_t(__builtin_ffsll(static_cast<long long int>(~index)));
        return (index == index_t(0)) ? std::nullopt
                                     : ((data[index - index_t(1)] == v) ? std::make_optional(index - index_t(1))
                                                                        : std::nullopt);
    }

public:
    [[nodiscard]] std::optional<index_t> constexpr impl(std::span<data_t const> const data,
        data_t const v) const noexcept override
    {
        return (data.size() > index_t(0)) ? branchless(data, v) : std::nullopt;
    }

    template <size_t N>
    [[nodiscard]] std::optional<index_t> constexpr impl(std::span<data_t, N> const data, data_t const v) const noexcept
    {
        return impl(data, v);
    }
};

template <typename D, typename I = std::size_t>
    requires std::is_unsigned_v<I> && std::is_integral_v<I>
struct eytzinger_prefetching : public binary_search<D, I> {
public:
    using data_t = binary_search<D, I>::data_t;
    using index_t = binary_search<D, I>::index_t;

private:
    [[nodiscard]] inline std::optional<index_t> constexpr prefetching(std::span<data_t const> const data,
        data_t const v) const noexcept
    {
        constexpr size_t block_size = size_t(CACHE_LINE_SIZE) / sizeof(data_t);
        index_t index = index_t(1);
        index_t const N = data.size();
        while (index <= N) {
            __builtin_prefetch(data.data() + (index - index_t(1)) * block_size, 0, 3);
            index = (index << 1u) | (data[index - index_t(1)] < v);
        }
        index >>= index_t(__builtin_ffsll(static_cast<long long int>(~index)));
        return (index == index_t(0)) ? std::nullopt
                                     : ((data[index - index_t(1)] == v) ? std::make_optional(index - index_t(1))
                                                                        : std::nullopt);
    }

public:
    [[nodiscard]] std::optional<index_t> constexpr impl(std::span<data_t const> const data,
        data_t const v) const noexcept override
    {
        return (data.size() > index_t(0)) ? prefetching(data, v) : std::nullopt;
    }

    template <size_t N>
    [[nodiscard]] std::optional<index_t> constexpr impl(std::span<data_t, N> const data, data_t const v) const noexcept
    {
        return impl(data, v);
    }
};
