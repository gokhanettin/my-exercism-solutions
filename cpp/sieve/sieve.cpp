#include "sieve.h"
#include <numeric>
#include <algorithm>

namespace sieve {

std::vector<int> primes(int n)
{
    std::vector<int> sieve(n + 1);
    std::vector<bool> marks(n + 1, true);

    std::iota(begin(sieve), end(sieve), 0);
    std::fill_n(begin(marks), 2, false);
    auto marker = 0;
    for (auto i = 2; i <= n; ++i) {
        if (marks[i]) {
            marker = i;
        }
        for (auto j = 2; j * marker <= n; ++j) {
            marks[j * marker] = false;
        }
    }

    sieve.erase(std::remove_if(begin(sieve), end(sieve),
        [&marks](auto number) { return !marks[number]; }), end(sieve));

    return sieve;
}

}  // namespace sieve
