#include <binary_search.hpp>

#include <cstdint>

//
// C wrappers for benchmarked functions
//

extern "C" {
bool dynamic_binary_search_power(size_t const N, int32_t const data[], int32_t const value)
{
    struct power<std::int32_t, std::size_t> bs { };
    std::span const wrapper(data, N);
    std::optional<std::size_t> const result = bs.impl(wrapper, value);
    return result.has_value();
}
}
