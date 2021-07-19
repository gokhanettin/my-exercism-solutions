#include "trinary.h"
#include <numeric>
#include <algorithm>

namespace trinary {

int to_decimal(const std::string& s)
{
    bool invalid = std::any_of(cbegin(s), cend(s),
        [] (auto c) {return c < '0' ||  c > '2'; });
    if (invalid) return 0;
    return std::accumulate(cbegin(s), cend(s), 0,
        [](auto acc, auto c) { return (c - '0') + acc * 3; });
}

}  // namespace trinary
