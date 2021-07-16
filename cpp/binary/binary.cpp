#include "binary.h"
#include <vector>
#include <numeric>
#include <algorithm>

namespace binary {

    int convert(const std::string& s)
    {
        if (std::find_if_not(begin(s), end(s),
         [](int c) { return c == '1' || c == '0'; }) != end(s)) {
             return 0;
        }
        std::vector<int> powers(s.size(), 2);
        powers[0] = 1;
        std::partial_sum(begin(powers), end(powers), begin(powers),
            std::multiplies<>{});

        std::transform(rbegin(s), rend(s), begin(powers), begin(powers),
            [](int c, int p) { return (c - '0') * p; });

        return std::accumulate(begin(powers), end(powers), 0);
    }

}  // namespace binary
