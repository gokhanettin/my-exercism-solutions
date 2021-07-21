#include "food_chain.h"

#include <sstream>
#include <vector>

namespace food_chain {
namespace {
struct descriptor {
    std::string what;
    std::string commentary;
    std::string why;
};

const std::vector<descriptor> descriptors{
    {"fly", "", ""},
    {"spider", "It wriggled and jiggled and tickled inside her.\n", "fly"},
    {"bird", "How absurd to swallow a bird!\n", "spider that wriggled and jiggled and tickled inside her"},
    {"cat", "Imagine that, to swallow a cat!\n", "bird"},
    {"dog", "What a hog, to swallow a dog!\n", "cat"},
    {"goat", "Just opened her throat and swallowed a goat!\n", "dog"},
    {"cow", "I don't know how she swallowed a cow!\n", "goat"},
    {"horse", "She's dead, of course!\n", ""},
};
}  // namespace
std::string verse(int ith)
{
    std::stringstream ss;

    int index = ith - 1;
    ss << "I know an old lady who swallowed a "
       << descriptors[index].what << ".\n"
       << descriptors[index].commentary;
    if (!descriptors[index].why.empty()) {
        for (auto i = index; i > 0; --i) {
            ss << "She swallowed the "
               << descriptors[i].what
               << " to catch the "
               << descriptors[i].why << ".\n";
        }
    }

    if (ith != static_cast<int>(descriptors.size())) {
        ss << "I don't know why she swallowed the fly. Perhaps she'll die.\n";
    }

    return ss.str();
}

std::string verses(int start, int end)
{
    std::string ret{""};
    for (auto i = start; i <= end; ++i) {
        ret.append(verse(i));
        ret.push_back('\n');
    }
    return ret;
}

std::string sing()
{
    return verses(1, 8);
}
}  // namespace food_chain
