#include "matching_brackets.h"

#include <stack>

namespace matching_brackets {
bool check(const std::string& s)
{
    std::stack<char> stack;
    const std::string open{"{[("};
    const std::string closed{"}])"};

    for (auto c : s) {
        size_t pos{0};
        if ((pos = open.find(c)) != std::string::npos) {
            stack.push(closed[pos]);
        } else if (closed.find(c) != std::string::npos) {
            if (stack.empty() || stack.top() != c) {
                return false;
            }
            stack.pop();
        }
    }
    return stack.empty();
}
} // namespace matching_brackets
