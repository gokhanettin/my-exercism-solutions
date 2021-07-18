#include "collatz_conjecture.h"
#include <stdexcept>

namespace collatz_conjecture {

int steps(int number)
{
    if (number <= 0) {
        throw std::domain_error("Bad number");
    }
    int step = 0;
    while (number != 1) {
        if (number % 2 == 0) {
            number /= 2;
        } else {
            number = 3 * number + 1;
        }
        ++step;
    }
    return step;
}

}  // namespace collatz_conjecture
