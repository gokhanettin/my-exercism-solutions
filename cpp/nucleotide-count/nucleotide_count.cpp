#include "nucleotide_count.h"
#include <stdexcept>

namespace nucleotide_count {

namespace {
const std::string nucleotides{"AGCT"};
bool is_valid(char c) {
    return nucleotides.find(c) != std::string::npos;
}
}

counter::counter(const std::string& s)
{
    for (auto nucleotide : nucleotides) {
        counts_[nucleotide] = 0;
    }
    for (auto c : s) {
        if (is_valid(c)) {
            ++counts_[c];
            } else {
                throw std::invalid_argument{"Invalid nucleotide"};
            }
    }
}

int counter::count(char c) const
{
    if (!is_valid(c)) {
        throw std::invalid_argument{"Invalid nucleotide query"};
    }
    return counts_.at(c);
}

std::map<char, int> counter::nucleotide_counts() const
{
    return counts_;
}
}  // namespace nucleotide_count
