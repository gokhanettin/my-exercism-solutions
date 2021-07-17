#include "reverse_string.h"
#include <algorithm>

namespace reverse_string {

std::string reverse_string(std::string s)
{
    std::reverse(begin(s), end(s));
    return s;
}

}  // namespace reverse_string
