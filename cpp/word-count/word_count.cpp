#include "word_count.h"
#include <regex>

namespace word_count {
std::map<std::string, int> words(const std::string& sentence)
{
    std::map<std::string, int> result;
    std::regex re{R"(\w+'?\w+|\w+)"};

    for (std::sregex_iterator it{begin(sentence), end(sentence), re};
         it != std::sregex_iterator{}; ++it) {
        std::string word{(*it)[0]};
        std::transform(begin(word), end(word), begin(word),
            [](auto c) { return std::tolower(c); });
        ++result[word];
    }
    return result;
}
} // namespace word_count
