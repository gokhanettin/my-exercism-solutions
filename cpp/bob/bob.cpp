#include "bob.h"
#include <algorithm>

namespace bob {
std::string hey(std::string input)
{
    input.erase(std::remove_if(begin(input), end(input),
                    [](auto c) { return std::isspace(c); }),
        end(input));

    bool question = input.back() == '?';
    bool any_alpha = false;
    bool any_lower = false;
    bool empty = input.empty();

    for (auto c : input) {
        if (std::isalpha(c)) {
            any_alpha = true;
        }
        if (std::islower(c)) {
            any_lower = true;
        }
    }
    bool all_caps = any_alpha && !any_lower;

    if (empty) {
        return "Fine. Be that way!";
    }

    if (all_caps && question) {
        return "Calm down, I know what I'm doing!";
    }

    if (all_caps) {
        return "Whoa, chill out!";
    }

    if (question) {
        return "Sure.";
    }

    return "Whatever.";
}
} // namespace bob
