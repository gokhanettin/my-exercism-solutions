#include "sum_of_multiples.h"

namespace sum_of_multiples {
int to(std::vector<int> divisors, int limit)
{
    int sum = 0;

    for (auto dividend = 0; dividend < limit; ++dividend) {
        for (auto divisor : divisors) {
            if (dividend % divisor == 0) {
                sum += dividend;
                break;
            }
        }
    }
    return sum;
}
}  // namespace sum_of_multiples
