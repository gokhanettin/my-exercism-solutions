#include "space_age.h"

namespace space_age {

namespace {
constexpr double EARTH_COEF = 1.0 / 31557600;
constexpr double MERCURY_COEF = EARTH_COEF / 0.2408467;
constexpr double VENUS_COEF = EARTH_COEF / 0.61519726;
constexpr double MARS_COEF = EARTH_COEF / 1.8808158;
constexpr double JUPYTER_COEF = EARTH_COEF / 11.862615;
constexpr double SATURN_COEF = EARTH_COEF / 29.447498;
constexpr double URANUS_COEF = EARTH_COEF / 84.016846;
constexpr double NEPTUNE_COEF = EARTH_COEF / 164.79132;
}

space_age::space_age(int64_t seconds)
    :seconds_{seconds} {}

int64_t space_age::seconds() const
{
    return seconds_;
}

double space_age::on_earth() const
{
    return seconds_ * EARTH_COEF;
}

double space_age::on_mercury() const
{
    return seconds_ * MERCURY_COEF;
}

double space_age::on_venus() const
{
    return seconds_ * VENUS_COEF;
}

double space_age::on_mars() const
{
    return seconds_ * MARS_COEF;
}

double space_age::on_jupiter() const
{
    return seconds_ * JUPYTER_COEF;
}

double space_age::on_saturn() const
{
    return seconds_ * SATURN_COEF;
}

double space_age::on_uranus() const
{
    return seconds_ * URANUS_COEF;
}

double space_age::on_neptune() const
{
    return seconds_ * NEPTUNE_COEF;
}
}  // namespace space_age
