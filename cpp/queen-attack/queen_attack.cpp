#include "queen_attack.h"
#include <stdexcept>

namespace queen_attack {

chess_board::chess_board()
    :chess_board{std::make_pair(0, 3), std::make_pair(7, 3)} {}

chess_board::chess_board(std::pair<int, int> white, std::pair<int, int> black)
    : white_{white}, black_{black}
{
    if (white_ == black_) {
        throw std::domain_error("Queens cannot occupy the same position");
    }
}

std::pair<int, int> chess_board::white() const
{
    return white_;
}

std::pair<int, int> chess_board::black() const
{
    return black_;
}

bool chess_board::can_attack() const
{
    int diffx = white_.first - black_.first;
    int diffy = white_.second - black_.second;

    if (diffx == 0 || diffy == 0) {
        return true;
    }

    diffx = diffx > 0 ? diffx : -diffx;
    diffy = diffy > 0 ? diffy : -diffy;

    if (diffx == diffy) {
        return true;
    }

    return false;
}

chess_board::operator std::string() const
{
    std::string board;
    for (auto i = 0; i < 8; ++i) {
        for (auto j = 0; j < 8; ++j) {
            if (i == white_.first && j == white_.second) {
                board.push_back('W');
            } else if (i == black_.first && j == black_.second) {
                board.push_back('B');
            } else {
                board.push_back('_');
            }
            board.push_back(' ');
        }
        board.pop_back();
        board.push_back('\n');
    }
    return board;
}

}  // namespace queen_attack
