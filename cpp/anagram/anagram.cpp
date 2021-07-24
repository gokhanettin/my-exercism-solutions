#include "anagram.h"
#include <algorithm>
#include <cctype>

namespace anagram {
namespace {
    anagram::histogram to_histogram(const std::string& text)
    {
        anagram::histogram hist{};
        for (auto c : text) {
            if (std::isalpha(c)) {
                ++hist[std::tolower(c) - 'a'];
            }
        }
        return hist;
    }

    std::string to_lowercase(std::string text)
    {
        std::transform(begin(text), end(text),
            begin(text), ::tolower);
        return text;
    }
}
anagram::anagram(std::string text)
    : text_{to_lowercase(text)}
{
    histogram_ = to_histogram(text_);
}

anagram::str_vector anagram::matches(const str_vector& candidates) const
{
    str_vector result;
    for (const auto& candidate : candidates) {
        if (match(candidate)) {
            result.push_back(candidate);
        }
    }
    return result;
}

bool anagram::match(const std::string& text) const
{
    std::string lowercase{to_lowercase(text)};
    return lowercase != text_ && to_histogram(lowercase) == histogram_;
}
} // namespace anagram
