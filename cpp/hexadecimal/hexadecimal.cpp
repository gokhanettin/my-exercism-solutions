#include "hexadecimal.h"
#include <numeric>

namespace hexadecimal {

unsigned int convert(const std::string& s)
{
    bool valid = true;
    const std::string digits{"0123456789ABCDEF"};
    size_t number = std::accumulate(begin(s), end(s), 0,
        [&](size_t acc, size_t digit) {
            size_t pos = digits.find(std::toupper(digit));
            if (pos == std::string::npos) {
                valid = false;
                return static_cast<size_t>(0);
            } else {
                return pos + (acc << 4);
            }
        }
    );

    if (!valid) {
        number = 0;
    }

    return number;
}

}  // namespace hexadecimal