#include "secret_handshake.h"
#include <algorithm>

namespace secret_handshake {

std::vector<std::string> commands(int input)
{
    std::vector<std::string> vec;
    if (input & (1 << 0)) {
        vec.push_back("wink");
    }
    if (input & (1 << 1)) {
        vec.push_back("double blink");
    }
    if (input & (1 << 2)) {
        vec.push_back("close your eyes");
    }
    if (input & (1 << 3)) {
        vec.push_back("jump");
    }
    if (input & (1 << 4)) {
        std::reverse(begin(vec), end(vec));
    }
    return vec;
}

}  // namespace secret_handshake
