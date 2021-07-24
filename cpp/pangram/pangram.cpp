#include "pangram.h"
#include <bitset>
#include <cctype>

namespace pangram {
bool is_pangram(const std::string& s)
{
    std::bitset<26> flags;

    for (auto c : s) {
        if (std::isalpha(c)) {
            flags.set(std::tolower(c) - 'a');
        }
    }
    return flags.all();
}
} // namespace pangram
