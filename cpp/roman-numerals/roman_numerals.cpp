#include "roman_numerals.h"
#include <string>
#include <utility>
#include <vector>

namespace roman_numerals {
std::string convert(int number)
{
    std::vector<std::pair<int, std::string>> pairs{
        {1000, "M"},
        {900, "CM"},
        {500, "D"},
        {400, "CD"},
        {100, "C"},
        {100, "C"},
        {90, "XC"},
        {50, "L"},
        {40, "XL"},
        {10, "X"},
        {9, "IX"},
        {8, "VIII"},
        {7, "VII"},
        {6, "VI"},
        {5, "V"},
        {4, "IV"},
        {3, "III"},
        {2, "II"},
        {1, "I"},
    };
    std::string ret{""};
    for (auto& pair : pairs) {
        while (number >= pair.first) {
            ret.append(pair.second);
            number -= pair.first;
        }
    }
    return ret;
}
} // namespace roman_numerals
