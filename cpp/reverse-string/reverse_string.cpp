#include "reverse_string.h"
#include <algorithm>

namespace reverse_string {

std::string reverse_string(const std::string& s)
{
    std::string tmp{s};
    std::reverse(begin(tmp), end(tmp));
    return tmp;

    // std::string tmp;
    // for (auto iter{crbegin(s)}; iter != std::crend(s); ++iter) {
    //     tmp.push_back(*iter);
    // }
    // return tmp;
}


}  // namespace reverse_string
