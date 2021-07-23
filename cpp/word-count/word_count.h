#if !defined(WORD_COUNT_H)
#define WORD_COUNT_H

#include <map>

namespace word_count {
std::map<std::string, int> words(const std::string& sentence);
}  // namespace word_count

#endif // WORD_COUNT_H