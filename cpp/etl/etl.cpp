#include "etl.h"
#include <cctype>

namespace etl {
NewFormat transform(const OldFormat& old)
{
    NewFormat result;

    for (auto& item : old) {
        auto& score = item.first;
        for (auto letter : item.second) {
            result[std::tolower(letter)] = score;
        }
    }
    return result;
}
}  // namespace etl
