#include "binary_search.h"
#include <stdexcept>

namespace binary_search {
std::size_t find(const std::vector<int>& v, int n)
{
    std::size_t low = 0;
    std::size_t high = v.size();
    while (high > low) {
        std::size_t middle = low + (high - low) / 2;
        if (n > v[middle]) {
            low = middle + 1;
        } else if (n < v[middle]) {
            high = middle;
        } else {
            return middle;
        }
    }
    throw std::domain_error{"Not found"};
}
} // namespace binary_search
