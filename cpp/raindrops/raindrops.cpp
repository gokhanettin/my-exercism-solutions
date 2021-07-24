#include "raindrops.h"
#include <utility>
#include <array>

namespace raindrops {
namespace {
const std::array<std::pair<int, std::string>, 3> pairs{{
    {3, "Pling"},
    {5, "Plang"},
    {7, "Plong"},
}};
}
std::string convert(int number)
{

    std::string ret;
    for (auto& pair : pairs) {
        if (number % pair.first == 0) {
            ret.append(pair.second);
        }
    }
     if (ret.empty()) {
         ret.append(std::to_string(number));
     }
     return ret;
}
} // namespace raindrops
