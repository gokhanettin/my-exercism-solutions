#if !defined(ROBOT_NAME_H)
#define ROBOT_NAME_H

#include <string>

namespace robot_name {


class robot {
    public:
        robot();
        void reset();
        std::string name() const;
    private:
        std::string name_;
};

}  // namespace robot_name

#endif // ROBOT_NAME_H