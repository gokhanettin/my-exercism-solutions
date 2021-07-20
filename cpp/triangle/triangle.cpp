#include "triangle.h"

#include <algorithm>
#include <cmath>
#include <stdexcept>
#include <vector>

namespace triangle {
namespace {
// Adapted from Python's isclose function.
bool is_close(double a, double b, double relative_tolerance = 1e-9,
              double absolute_tolerance = 0.0)
{
    auto max = std::max(std::fabs(a), std::fabs(b));
    auto diff = std::fabs(a - b);
    return diff <= std::max(relative_tolerance * max, absolute_tolerance);
}
}  // namespace

flavor kind(double a, double b, double c)
{
    if (a == 0.0 || b == 0.0 || c == 0.0) {
        throw std::domain_error{"A side cannot be zero"};
    }

    std::vector<double> sides{a, b, c};
    std::sort(begin(sides), end(sides));
    if (sides[2] > sides[0] + sides[1]) {
        throw std::domain_error{"Sides do not represent a triangle"};
    }

    flavor ret{flavor::scalene};
    sides.erase(std::unique(begin(sides), end(sides),
                            [](auto a, auto b) { return is_close(a, b); }),
                end(sides));
    if (sides.size() == 1) {
        ret = flavor::equilateral;
    } else if (sides.size() == 2) {
        ret = flavor::isosceles;
    }
    return ret;
}

}  // namespace triangle
