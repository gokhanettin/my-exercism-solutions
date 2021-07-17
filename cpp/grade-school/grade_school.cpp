#include "grade_school.h"
#include <algorithm>

namespace grade_school {
void school::add(const std::string& name, int grade)
{
    auto& names = roster_[grade];
    auto iter = std::upper_bound(begin(names), end(names), name);
    names.insert(iter, std::move(name));
}
school::name_vector school::grade(int grade) const
{
    const auto iter = roster_.find(grade);
    if (iter != end(roster_)) {
        return iter->second;
    }

    return {};
}
school::roster_map school::roster() const
{
    return roster_;
}
}  // namespace grade_school
