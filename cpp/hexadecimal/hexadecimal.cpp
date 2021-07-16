#include "hexadecimal.h"

namespace hexadecimal {

unsigned int convert(const std::string& s)
{
    unsigned int ret = 0;
    const std::string digits = "0123456789ABCDEF";

    for (size_t i{0}; i < s.size(); ++i) {
        char c = std::toupper(s[i]);
        size_t pos = digits.find(c);
        if (pos != std::string::npos) {
            ret += pos * (1U << (4 * (s.size() - i - 1)));
        } else {
            ret = 0;
            break;
        }
    }
    return ret;
}

}  // namespace hexadecimal
