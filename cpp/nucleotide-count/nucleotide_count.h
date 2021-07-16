#if !defined(NUCLEOTIDE_COUNT_H)
#define NUCLEOTIDE_COUNT_H

#include <string>
#include <map>

namespace nucleotide_count {
class counter {
    public:
        counter(const std::string& s);
        int count(char c) const;
        std::map<char, int> nucleotide_counts() const;
    private:
        std::map<char, int> counts_;
    };
}  // namespace nucleotide_count

#endif // NUCLEOTIDE_COUNT_H