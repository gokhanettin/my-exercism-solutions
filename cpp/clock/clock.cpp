#include "clock.h"
#include <iomanip>
#include <sstream>
#include <cassert>

namespace date_independent {

namespace {
    int rem_euclid(int dividend, int divisor) {
        assert(divisor != 0);
        return ((dividend % divisor) + std::abs(divisor)) % std::abs(divisor);
    }

    int div_euclid(int dividend, int divisor) {
        return (dividend - rem_euclid(dividend, divisor)) / divisor;
    }
}

clock::clock(int hour, int minute)
    :hour_{hour}, minute_{minute} {}

void clock::normalize()
{
    hour_ += div_euclid(minute_, 60);
    minute_ = rem_euclid(minute_, 60);
    hour_ = rem_euclid(hour_, 24);
}

clock clock::at(int hour, int minute)
{
    clock ret{hour, minute};
    ret.normalize();
    return ret;
}

clock::operator std::string() const
{
    std::stringstream ss;

    ss << std::setfill('0') << std::setw(2) << hour_;
    ss << std::setw(1) << ":";
    ss << std::setw(2) << minute_;
    return ss.str();
}

clock clock::plus(int amount) const
{
    clock ret{*this};
    ret.minute_ += amount;
    ret.normalize();
    return ret;
}

bool operator==(const clock& lhs, const clock& rhs)
{
    return lhs.hour_ == rhs.hour_ && lhs.minute_ == rhs.minute_;
}
bool operator!=(const clock& lhs, const clock& rhs)
{
    return !(lhs == rhs);
}
}  // namespace date_independent
