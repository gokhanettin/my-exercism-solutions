#include "grains.h"

namespace grains {
unsigned long long square(int nth)
{
    return 1ULL << (nth - 1);
}
unsigned long long total()
{
    return (((1ULL << 63) - 1) << 1) + 1;
}
}  // namespace grains
