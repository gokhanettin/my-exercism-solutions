#include "bob.h"
#include <algorithm>

namespace bob {
std::string hey(std::string input)
{
    input.erase(std::remove_if(begin(input), end(input),
                    [](auto c) { return std::isspace(c); }),
        end(input));

    bool question = input.back() == '?';
    bool empty = input.empty();
    input.erase(std::remove_if(begin(input), end(input),
                    [](auto c) { return !std::isalpha(c); }),
        end(input));
    bool all_caps = !input.empty() && std::all_of(cbegin(input), cend(input),
        [](auto c) { return std::isupper(c); });

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
