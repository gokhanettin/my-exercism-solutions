#if !defined(ROBOT_SIMULATOR_H)
#define ROBOT_SIMULATOR_H

#include <string>

namespace robot_simulator {

enum class Bearing {
    NORTH,
    EAST,
    SOUTH,
    WEST
};

class Robot {
    public:
        Robot();
        Robot(std::pair<int, int> position, Bearing bearing);
        std::pair<int, int> get_position() const;
        Bearing get_bearing() const;
        void turn_left();
        void turn_right();
        void advance();
        void execute_sequence(const std::string& sequence);
    private:
        std::pair<int, int> position_;
        Bearing bearing_;
};

}  // namespace robot_simulator

#endif // ROBOT_SIMULATOR_H