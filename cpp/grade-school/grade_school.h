#if !defined(GRADE_SCHOOL_H)
#define GRADE_SCHOOL_H

#include <string>
#include <vector>
#include <map>

namespace grade_school {

class school {
    public:
        using name_vector = std::vector<std::string>;
        using roster_map = std::map<int, name_vector>;

        school() = default;
        void add(const std::string& name, int grade);
        name_vector grade(int grade) const;
        roster_map roster() const;
    private:
        roster_map roster_;
};

}  // namespace grade_school

#endif // GRADE_SCHOOL_H