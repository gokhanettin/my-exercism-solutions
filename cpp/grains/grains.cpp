#include "grains.h"

namespace grains {
unsigned long long square(int nth)
{
    return 1ULL << (nth - 1);
}
unsigned long long total()
{
    // 2^64 - 1
    return ~0ULL;
}
}  // namespace grains
