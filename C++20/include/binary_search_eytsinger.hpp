#pragma once

#include <bit>
#include <iostream>
#include <span>

//
// Helper functions for binary search implementations
//

template <typename D>
struct eytzinger_base {
public:
    using data_t = D;

private:
    [[nodiscard]] static constexpr std::size_t eytzinger_layout(std::span<data_t> eytzinger, std::span<data_t const> const monotonic, std::size_t i, std::size_t const k) noexcept
    {
        data_t* const buffer = std::prev(eytzinger.data());
        if (k <= eytzinger.size()) {
            i = eytzinger_layout(eytzinger, monotonic, i, k << 1u);
            buffer[k] = monotonic[i++];
            i = eytzinger_layout(eytzinger, monotonic, i, (k << 1u) | 1u);
        }
        return i;
    }

public:
    [[nodiscard]] static constexpr bool eytzinger_layout(std::span<data_t> eytzinger, std::span<data_t const> const monotonic) noexcept
    {
        if (eytzinger.size() != monotonic.size()) {
            return false;
        }

        std::size_t const len = eytzinger_layout(eytzinger, monotonic, 0u, 1u);

        return len == monotonic.size();
    }
};

//
// Eytzinger layout based implementations of binary searches
//

template <typename D, typename I = std::size_t>
    requires std::is_unsigned_v<I> && std::is_integral_v<I>
struct eytzinger_hintless : public binary_search<D, I>, public eytzinger_base<D> {
public:
    using data_t = binary_search<D, I>::data_t;
    using index_t = binary_search<D, I>::index_t;

private:
    [[nodiscard]] inline std::optional<index_t> constexpr hintless(std::span<data_t const> const data,
        data_t const v) const noexcept
    {
        data_t const* const buffer = std::prev(data.data());
        index_t const N = static_cast<index_t>(data.size());
        index_t index = index_t(1);
        while (index <= N) {
            if (buffer[index] < v) {
                index <<= 1u;
                index |= 1u;
            } else {
                index <<= 1u;
            }
        }
        index >>= index_t(__builtin_ffsll(static_cast<long long int>(~index)));
        return (index == index_t(0) || (buffer[index] != v)) ? std::nullopt
                                                             : std::make_optional(index - index_t(1));
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
struct eytzinger_branchless : public binary_search<D, I>, public eytzinger_base<D> {
public:
    using data_t = binary_search<D, I>::data_t;
    using index_t = binary_search<D, I>::index_t;

private:
    [[nodiscard]] inline std::optional<index_t> constexpr branchless(std::span<data_t const> const data,
        data_t const v) const noexcept
    {
        data_t const* const buffer = std::prev(data.data());
        index_t const N = static_cast<index_t>(data.size());
        index_t index = index_t(1);
        while (index <= N) {
            index = (index << 1u) | (buffer[index] < v);
        }
        index >>= index_t(__builtin_ffsll(static_cast<long long int>(~index)));
        return (index == index_t(0) || (buffer[index] != v)) ? std::nullopt
                                                             : std::make_optional(index - index_t(1));
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
struct eytzinger_prefetching : public binary_search<D, I>, public eytzinger_base<D> {
public:
    using data_t = binary_search<D, I>::data_t;
    using index_t = binary_search<D, I>::index_t;

private:
    [[nodiscard]] inline std::optional<index_t> constexpr prefetching(std::span<data_t const> const data,
        data_t const v) const noexcept
    {
        data_t const* const buffer = std::prev(data.data());
        index_t const N = static_cast<index_t>(data.size());
        index_t index = index_t(1);
        __builtin_prefetch(&buffer[index]);
        while (index <= N) {
            __builtin_prefetch(&buffer[index << 1u]);
            index = (index << 1u) | (buffer[index] < v);
        }
        index >>= index_t(__builtin_ffsll(static_cast<long long int>(~index)));
        return (index == index_t(0) || (buffer[index] != v)) ? std::nullopt
                                                             : std::make_optional(index - index_t(1));
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
