#include "phone_number.h"

#include <algorithm>
#include <stdexcept>

namespace phone_number {

phone_number::phone_number(const std::string& s)
    : phone_{s}
{
    format(phone_);
}

phone_number::operator std::string() const
{
    return phone_;
}

std::string phone_number::number() const
{
    std::string number{phone_};
    number.erase(std::remove_if(begin(number), end(number),
                     [](auto c) { return !std::isdigit(c); }),
        end(number));
    return number;
}

std::string phone_number::area_code() const
{
    return phone_.substr(1, 3);
}

void phone_number::format(std::string& s)
{
    s.erase(std::remove_if(begin(s), end(s),
                [](auto c) { return !std::isdigit(c); }),
        end(s));

    if (s.size() == 11) {
        if (s[0] != '1') {
            throw std::domain_error{"Country code is not 1: " + s};
        }
        s = s.substr(1);
    }

    if (s.size() != 10) {
        throw std::domain_error{"Number does not contain 10 digits: " + s};
    }

    if (s[0] < '2' || s[3] < '2') {
        throw std::domain_error{"Bad code: " + s};
    }
    s.insert(0, "(");
    s.insert(4, ") ");
    s.insert(9, "-");
}

std::string phone_number(std::string s)
{
    return s;
}
} // namespace phone_number
