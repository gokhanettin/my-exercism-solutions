#include "atbash_cipher.h"

#include <numeric>

namespace atbash_cipher {

namespace {
char transpose(char c)
{
    if (std::isalpha(c)) {
        return 'a' + 'z' - c;
    }

    return c;
}
}  // namespace
std::string encode(const std::string& s)
{
    int counter{0};
    return std::accumulate(cbegin(s), cend(s), std::string{""},
        [&counter](auto acc, auto c) {
            if (std::isalnum(c)) {
                if (counter > 0 && counter % 5 == 0) {
                    acc.push_back(' ');
                }
                acc.push_back(transpose(std::tolower(c)));
                ++counter;
            }
            return acc;
        });
}

std::string decode(const std::string& s)
{
    return std::accumulate(cbegin(s), cend(s), std::string{""},
        [](auto acc, auto c) {
            if (std::isalnum(c)) {
                acc.push_back(transpose(c));
            }
            return acc;
        });
}
}  // namespace atbash_cipher
