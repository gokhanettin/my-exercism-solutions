#include "anagram.h"
#include <algorithm>
#include <cctype>

namespace anagram {
namespace {
    std::string to_lowercase(std::string text)
    {
        std::transform(begin(text), end(text),
            begin(text), ::tolower);
        return text;
    }
}
anagram::anagram(const std::string& text)
    : text_{to_lowercase(text)}
{
}

std::vector<std::string> anagram::matches(
    const std::vector<std::string>& candidates) const
{
    std::vector<std::string> result;
    std::copy_if(cbegin(candidates), cend(candidates),
        std::back_inserter(result),
        [this](auto candidate) { return match(candidate); });
    return result;
}

bool anagram::match(const std::string& text) const
{
    std::string lowercase{to_lowercase(text)};

    if (lowercase.size() != text_.size())
        return false;
    if (lowercase == text_)
        return false;
    return std::is_permutation(cbegin(lowercase), cend(lowercase),
        cbegin(text_));
}
} // namespace anagram
