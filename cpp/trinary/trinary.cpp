#include "trinary.h"
#include <numeric>

namespace trinary {

int to_decimal(const std::string& s)
{
    int decimal{0};
    bool valid{true};

    decimal = std::accumulate(cbegin(s), cend(s), 0,
        [&valid](auto acc, auto c) {
            int digit = c - '0';
            if (digit > 3) valid = false;
            return digit + acc * 3;
        }
    );

    if (!valid) decimal = 0;
    return decimal;
}

}  // namespace trinary
