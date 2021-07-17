#include "robot_name.h"
#include <vector>
#include <random>
#include <cstdint>
#include <algorithm>

namespace robot_name {

namespace {
class unique_number_generator {
    public:
        unique_number_generator(uint32_t n);
        uint32_t unique_number();
    private:
        std::vector<uint32_t> numbers_;
        size_t index_;
};

unique_number_generator::unique_number_generator(uint32_t n)
    :numbers_(n), index_(0)
{
    std::random_device rd;
    std::mt19937 g(rd());
    // Generate unique numbers 0, 1, 2, 3, ..., n
    std::iota(begin(numbers_), end(numbers_), 0);
    // shuffle the numbers
    std::shuffle(begin(numbers_), end(numbers_), g);
}

uint32_t unique_number_generator::unique_number()
{
    return numbers_.at(index_++);
}

}

robot::robot()
    :name_(5, 0)
{
    reset();
}

void robot::reset()
{
    thread_local static unique_number_generator gen(26 * 26 * 10 * 10 *10);
    uint32_t id = gen.unique_number();

    name_[0] = ('A' + id % 26); id /= 26;
    name_[1] = ('A' + id % 26); id /= 26;
    name_[2] = ('0' + id % 10); id /= 10;
    name_[3] = ('0' + id % 10); id /= 10;
    name_[4] = ('0' + id % 10); id /= 10;
}

std::string robot::name() const
{
    return name_;
}

}  // namespace robot_name
