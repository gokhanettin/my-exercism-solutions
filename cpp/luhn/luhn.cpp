#include "luhn.h"
#include <algorithm>

namespace luhn {
    bool valid(const std::string& s)
    {
        std::string tmp{s};

        tmp.erase(std::remove(begin(tmp), end(tmp), ' '), end(tmp));
        if (tmp.size() <= 1) {
            return false;
        }

        bool should_double = false;
        int sum = 0;
        for (auto iter{crbegin(tmp)}; iter < crend(tmp); ++iter) {
            if (!std::isdigit(*iter)) {
                return false;
            }

            int digit = *iter - '0';
            if (should_double) {
                digit *= 2;
                digit = digit <= 9 ? digit : digit - 9;
            }
            should_double = !should_double;
            sum += digit;
        }

        return sum % 10 == 0;
    }
}  // namespace luhn
