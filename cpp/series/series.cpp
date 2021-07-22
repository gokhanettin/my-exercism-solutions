#include "series.h"
#include <algorithm>
#include <stdexcept>

namespace series {
std::vector<int> digits(const std::string& s)
{
    std::vector<int> result;
    std::transform(cbegin(s), cend(s), std::back_inserter(result),
        [](auto c) { return c - '0'; });
    return result;
}

std::vector<std::vector<int>> slice(const std::string& s, size_t n)
{
    if (n > s.size()) {
        throw std::domain_error{"Not enough digits to slice"};
    }
    auto v = digits(s);
    std::vector<std::vector<int>> result;
    for (size_t i = 0; i + n <= v.size(); ++i) {
        result.emplace_back(std::next(std::begin(v), i),
            std::next(std::begin(v), i + n));
    }
    return result;
}
} // namespace series
