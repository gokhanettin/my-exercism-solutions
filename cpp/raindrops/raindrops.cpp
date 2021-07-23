#include "raindrops.h"
#include <sstream>

namespace raindrops {
std::string convert(int number)
{
    std::stringstream ss;
    if (number % 3 == 0) {
        ss << "Pling";
    }
    if (number % 5 == 0) {
        ss << "Plang";
    }
    if (number % 7 == 0) {
        ss << "Plong";
    }
    if (!ss.rdbuf()->in_avail()) {
        ss << number;
    }
    return ss.str();
}
} // namespace raindrops
