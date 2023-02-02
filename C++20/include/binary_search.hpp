#pragma once

#include <array>
#include <limits>
#include <optional>
#include <span>

//
// Dynamic and static interfaces for binary search implementations
//

template <typename D, typename I>
struct binary_search {
public:
    using data_t = D;
    using index_t = I;

public:
    [[nodiscard]] virtual std::optional<index_t> constexpr impl([[maybe_unused]] std::span<data_t const> const data,
        [[maybe_unused]] data_t const v) const
    {
        return std::nullopt;
    }

    template <std::size_t N>
    [[nodiscard]] inline std::optional<index_t> constexpr impl(std::array<data_t const, N> const data,
        data_t const v) const;
};

#include "binary_search_alternative.hpp"
#include "binary_search_power.hpp"
#include "binary_search_range.hpp"
#include "binary_search_traditional1.hpp"
#include "binary_search_traditional2.hpp"
