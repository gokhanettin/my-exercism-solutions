#include "hamming.h"
#include <stdexcept>
#include <numeric>
#include <functional>

namespace hamming {

int compute(const std::string& s1, const std::string& s2)
{
    if (s1.size() != s2.size()) {
        throw std::domain_error("Size mismatch");
    }

    return std::inner_product(begin(s1), end(s1),
        begin(s2), 0, std::plus<>{}, std::not_equal_to<>{});
}

}  // namespace hamming
