#include "armstrong_numbers.h"
#include <vector>
#include <numeric>
#include <cmath>

namespace armstrong_numbers {
bool is_armstrong_number(int number)
{
    std::vector<int> digits;
    int tmp = number;
    while (tmp) {
        digits.push_back(tmp % 10);
        tmp /= 10;
    }

    size_t power = digits.size();
    return std::accumulate(begin(digits), end(digits), 0,
        [power](auto acc, auto digit) {
            return acc + static_cast<int>(pow(digit, power));
        }
    ) == number;
}
}  // namespace armstrong_numbers
