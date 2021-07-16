#if !defined(ALL_YOUR_BASE_H)
#define ALL_YOUR_BASE_H

#include <vector>
#include <cstdint>

namespace all_your_base {
    std::vector<uint32_t> convert(int from,
        std::vector<uint32_t> digits, int to);
}  // namespace all_your_base

#endif // ALL_YOUR_BASE_H