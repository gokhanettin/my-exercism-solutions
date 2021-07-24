#if !defined(ANAGRAM_H)
#define ANAGRAM_H

#include <string>
#include <vector>
#include <array>

namespace anagram {
class anagram
{
public:
    using str_vector = std::vector<std::string>;
    using histogram = std::array<int, 26>;

    explicit anagram(std::string text);
    str_vector matches(const str_vector& candidates) const;
private:
    bool match(const std::string& text) const;
    std::string text_;
    histogram histogram_;


};

}  // namespace anagram

#endif // ANAGRAM_H