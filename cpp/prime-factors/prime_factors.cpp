#include "prime_factors.h"

namespace prime_factors {
std::vector<int> of(int n)
{
    std::vector<int> factors;
    int candidate = 2;
    while (n >= candidate) {
        if (n % candidate == 0) {
            factors.push_back(candidate);
            n /= candidate;
        } else {
            candidate += 1;
        }
    }
    return factors;
}
}  // namespace prime_factors
