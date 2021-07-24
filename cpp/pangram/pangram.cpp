#include "pangram.h"
#include <algorithm>
#include <array>

namespace pangram {
bool is_pangram(const std::string& s)
{
    std::array<int, 26> histogram;

    histogram.fill(0);
    for (auto c : s) {
        c = std::tolower(c);
        if (c >= 'a' && c <= 'z') {
            ++histogram[c - 'a'];
        }
    }
    return !std::any_of(cbegin(histogram), cend(histogram),
        [](auto val) { return val == 0; });
}
} // namespace pangram
