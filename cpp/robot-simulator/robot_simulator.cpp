#include "robot_simulator.h"
#include <algorithm>

namespace robot_simulator {

Robot::Robot()
    :Robot{std::pair<int, int>{0, 0}, Bearing::NORTH} {}

Robot::Robot(std::pair<int, int> position, Bearing bearing)
    :position_{position}, bearing_{bearing} {}

std::pair<int, int> Robot::get_position() const
{
    return position_;
}

Bearing Robot::get_bearing() const
{
    return bearing_;
}

void Robot::turn_left()
{
    switch (bearing_) {
        case Bearing::NORTH:
            bearing_ = Bearing::WEST;
            break;
        case Bearing::EAST:
            bearing_ = Bearing::NORTH;
            break;
        case Bearing::SOUTH:
            bearing_ = Bearing::EAST;
            break;
        case Bearing::WEST:
            bearing_ = Bearing::SOUTH;
            break;
    }
}

void Robot::turn_right()
{
    switch (bearing_) {
        case Bearing::NORTH:
            bearing_ = Bearing::EAST;
            break;
        case Bearing::EAST:
            bearing_ = Bearing::SOUTH;
            break;
        case Bearing::SOUTH:
            bearing_ = Bearing::WEST;
            break;
        case Bearing::WEST:
            bearing_ = Bearing::NORTH;
            break;
    }
}

void Robot::advance()
{
    auto& x = position_.first;
    auto& y = position_.second;
    switch (bearing_) {
        case Bearing::NORTH:
            ++y;
            break;
        case Bearing::EAST:
            ++x;
            break;
        case Bearing::SOUTH:
            --y;
            break;
        case Bearing::WEST:
            --x;
            break;
    }
}

void Robot::execute_sequence(const std::string& sequence)
{
    std::for_each(begin(sequence), end(sequence),
        [this](int c) {
            switch (c) {
                case 'A':
                    advance();
                    break;
                case 'R':
                    turn_right();
                    break;
                case 'L':
                    turn_left();
                    break;
            }
        }
    );
}

}  // namespace robot_simulator
