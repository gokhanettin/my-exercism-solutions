#if !defined(ETL_H)
#define ETL_H

#include <map>
#include <vector>

namespace etl {
using NewFormat = std::map<char, int>;
using OldFormat = std::map<int, std::vector<char>>;
NewFormat transform(const OldFormat& old);
}  // namespace etl

#endif // ETL_H