#include "triangle.h"
#include <vector>
#include <algorithm>
#include <stdexcept>


namespace triangle {

flavor kind(double a, double b, double c) {
    flavor ret;
    std::vector<double> sides{a, b, c};

    std::sort(begin(sides), end(sides));
    if (sides[2] < sides[0] + sides[1]) {
        sides.erase(std::unique(begin(sides), end(sides)), end(sides));
        if (sides.size() == 1) {
            ret = flavor::equilateral;
        } else if (sides.size() == 2) {
            ret = flavor::isosceles;
        } else {
            ret = flavor::scalene;
        }
    } else {
        throw std::domain_error{"Sides do not represent a triangle"};
    }
    return ret;
}

}  // namespace triangle
