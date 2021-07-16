#include "all_your_base.h"
#include <numeric>
#include <cstdint>
#include <stdexcept>
#include <algorithm>
#include <functional>

namespace all_your_base {
    std::vector<uint32_t> convert(int from,
        std::vector<uint32_t> digits, int to)
    {
        if (from < 2 || to < 2) {
            throw std::invalid_argument{"Bad base"};
        }

        for (auto digit : digits) {
            if (digit >= static_cast<uint32_t>(from)) {
                throw std::invalid_argument{"Inconsistent digit with base"};
            }
        }

        uint32_t number = std::accumulate(begin(digits), end(digits), 0,
            [from](uint32_t acc, uint32_t digit) {
                return digit + acc * from;
            }
        );

        std::vector<uint32_t> result;
        while (number) {
            result.push_back(number % to);
            number = number / to;
        }

        std::reverse(begin(result), end(result));
        return result;
    }
}  // namespace all_your_base
