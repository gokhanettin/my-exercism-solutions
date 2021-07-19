#include "space_age.h"

namespace space_age {

namespace {
constexpr double earth_years_per_second = 1.0 / 31557600;
constexpr double mercury_years_per_second = earth_years_per_second / 0.2408467;
constexpr double venus_yearss_per_second = earth_years_per_second / 0.61519726;
constexpr double mars_years_per_second = earth_years_per_second / 1.8808158;
constexpr double jupyter_years_per_second = earth_years_per_second / 11.862615;
constexpr double saturn_years_per_second = earth_years_per_second / 29.447498;
constexpr double uranus_years_per_second = earth_years_per_second / 84.016846;
constexpr double neptune_years_per_second = earth_years_per_second / 164.79132;
}

space_age::space_age(std::int64_t seconds)
    :seconds_{seconds} {}

std::int64_t space_age::seconds() const
{
    return seconds_;
}

double space_age::on_earth() const
{
    return seconds_ * earth_years_per_second;
}

double space_age::on_mercury() const
{
    return seconds_ * mercury_years_per_second;
}

double space_age::on_venus() const
{
    return seconds_ * venus_yearss_per_second;
}

double space_age::on_mars() const
{
    return seconds_ * mars_years_per_second;
}

double space_age::on_jupiter() const
{
    return seconds_ * jupyter_years_per_second;
}

double space_age::on_saturn() const
{
    return seconds_ * saturn_years_per_second;
}

double space_age::on_uranus() const
{
    return seconds_ * uranus_years_per_second;
}

double space_age::on_neptune() const
{
    return seconds_ * neptune_years_per_second;
}
}  // namespace space_age
