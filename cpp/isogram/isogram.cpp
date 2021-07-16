#include "isogram.h"
#include <unordered_map>

namespace isogram {
    bool is_isogram(const std::string& s)
    {
        using key = int;
        using value = int;
        std::unordered_map<key, value> hash;

        bool ret = true;
        for (auto c : s) {
            if (c != ' ' && c!= '-') {
                c = std::tolower(c);
                ++hash[c];

                if (hash[c] > 1) {
                    ret = false;
                    break;
                }
            }
        }
        return ret;
    }
}  // namespace isogram
