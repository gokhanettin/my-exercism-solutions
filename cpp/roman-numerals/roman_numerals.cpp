#include "roman_numerals.h"
#include <sstream>
#include <string>
#include <unordered_map>

namespace roman_numerals {
std::string convert(int number)
{
    std::unordered_map<int, std::stringstream> hash;
    auto f = [](std::string&& s, int n, auto& hash) {
        if (s.size() > 0) {
            hash[n] << s[0];
            hash[2 * n] << s[0] << s[0];
            hash[3 * n] << s[0] << s[0] << s[0];
        }
        if (s.size() > 1) {
            hash[4 * n] << s[0] << s[1];
            hash[5 * n] << s[1];
            hash[6 * n] << s[1] << s[0];
            hash[7 * n] << s[1] << s[0] << s[0];
            hash[8 * n] << s[1] << s[0] << s[0] << s[0];
        }
        if (s.size() > 2) {
            hash[9 * n] << s[0] << s[2];
        }
    };

    f("IVX", 1, hash);
    f("XLC", 10, hash);
    f("CDM", 100, hash);
    f("M", 1000, hash);

    std::string ret{""};
    for (auto i = 1; number; number /= 10, i *= 10) {
        ret = hash[(number % 10) * i].str() + ret;
    }

    return ret;
}
} // namespace roman_numerals
