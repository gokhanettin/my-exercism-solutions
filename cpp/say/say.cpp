#include "say.h"
#include <unordered_map>
#include <vector>
#include <stdexcept>

namespace say {
namespace {
std::string chunk_in_english(long long number) {
    static const std::unordered_map< long long, std::string> map{
        {0ULL,   "zero"},
        {1ULL,   "one"},
        {2ULL,   "two"},
        {3ULL,   "three"},
        {4ULL,   "four"},
        {5ULL,   "five"},
        {6ULL,   "six"},
        {7ULL,   "seven"},
        {8ULL,   "eight"},
        {9ULL,   "nine"},
        {10ULL,  "ten"},
        {11ULL,  "eleven"},
        {12ULL,  "twelve"},
        {13ULL,  "thirteen"},
        {14ULL,  "fourteen"},
        {15ULL,  "fifteen"},
        {16ULL,  "sixteen"},
        {17ULL,  "seventeen"},
        {18ULL,  "eighteen"},
        {19ULL,  "nineteen"},
        {20ULL,  "twenty"},
        {30ULL,  "thirty"},
        {40ULL,  "forty"},
        {50ULL,  "fifty"},
        {60ULL,  "sixty"},
        {70ULL,  "seventy"},
        {80ULL,  "eighty"},
        {90ULL,  "ninety"},
    };

    std::string ret;
    if (map.find(number) != end(map)) {
        ret.append(map.at(number));
    } else {
        if (number >= 100) {
            ret.append(map.at(number / 100));
            ret.push_back(' ');
            ret.append("hundred");
            number %= 100;
            if (number) {
                ret.push_back(' ');
            }
        }
        if (map.find(number) != end(map)) {
            if (number) {
                ret.append(map.at(number));
            }
        } else {
            ret.append(map.at((number / 10) * 10));
            ret.push_back('-');
            ret.append(map.at(number % 10));
        }
    }
    return ret;
}
}

std::string in_english(long long number)
{
    if (number < 0 || number > 999'999'999'999) {
        throw std::domain_error("Invalid number");
    }

    if (number < 1000) {
        return chunk_in_english(number);
    }

    std::vector< long long> chunks;
    while (number) {
        chunks.push_back(number % 1000);
        number /= 1000;
    }

    const std::vector<std::string> scales{
        "",
        "thousand",
        "million",
        "billion"
    };

    std::string ret;
    int i = chunks.size() - 1;
    for (auto it{crbegin(chunks)}; it != crend(chunks); ++it) {
        if (*it) {
            if (!ret.empty()) ret.push_back(' ');
            ret.append(chunk_in_english(*it));
            if (i > 0) ret.push_back(' ');
            ret.append(scales[i]);
        }
        --i;
    }

    return ret;
}
}  // namespace say
