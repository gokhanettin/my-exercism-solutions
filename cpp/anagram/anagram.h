#if !defined(ANAGRAM_H)
#define ANAGRAM_H

#include <string>
#include <vector>

namespace anagram {
class anagram {
public:
    explicit anagram(const std::string& text);
    std::vector<std::string> matches(
        const std::vector<std::string>& candidates) const;

private:
    bool match(const std::string& text) const;
    std::string text_;
};

} // namespace anagram

#endif // ANAGRAM_H