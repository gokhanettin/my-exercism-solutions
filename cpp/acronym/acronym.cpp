#include "acronym.h"
#include <numeric>

namespace acronym {

std::string acronym(const std::string& s)
{
    std::string acronym;

    auto iter = begin(s);
    while (iter != end(s)) {
        if (iter == begin(s)) {
            acronym.push_back(*iter);
        }

        if (*iter == ' ' || *iter == '-') {
            ++iter;
            if (std::isalpha(*iter)) {
                acronym.push_back(std::toupper(*iter));
            }
        }
        ++iter;
    }
    return acronym;
}

}  // namespace acronym
