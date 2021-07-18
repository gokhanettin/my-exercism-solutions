#include "sieve.h"
#include <unordered_map>

namespace sieve {

namespace {
class prime_generator {
    public:
        explicit prime_generator(int upto = std::numeric_limits<int>::max());
        int operator() ();
    private:
        void mark_composite(int times, int prime);
        bool is_prime(int number);
        int next();

        int upto_;
        int curr_;
        std::unordered_map<int, int> hash_;
};

prime_generator::prime_generator(int upto)
    :upto_{upto}, curr_{2} {}

void prime_generator::mark_composite(int times, int prime)
{
    while (hash_.find(times * prime) != end(hash_)) {
        ++times;
    }
    hash_[times * prime] = prime;
}

bool prime_generator::is_prime(int number)
{
    bool ret = false;
    auto pair = hash_.find(number);
    if (pair != end(hash_)) {
        // number is not prime
        int prime = pair->second;
        hash_.erase(pair);
        mark_composite(number / prime + 1, prime);
    } else {
        // number is prime
        mark_composite(2, number);
        ret = true;
    }
    return ret;
}

int prime_generator::next()
{
    int prime = 0;
    for (auto n = curr_; n <= upto_; ++n) {
        if (is_prime(n)) {
            prime = n;
            curr_ = prime + 1;
            break;
        }
    }
    return prime;
}

int prime_generator::operator() () {
    return next();
}
}

std::vector<int> primes(int n)
{
    prime_generator generator{n};
    std::vector<int> prime_numbers;

    while (auto prime = generator()) {
        prime_numbers.push_back(prime);
    }
    return prime_numbers;
}

}  // namespace sieve
