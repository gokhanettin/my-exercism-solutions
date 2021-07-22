#if !defined(SERIES_H)
#define SERIES_H

#include <string>
#include <vector>

namespace series {
std::vector<int> digits(const std::string& s);
std::vector<std::vector<int>> slice(const std::string& s, size_t n);
} // namespace series

#endif // SERIES_H