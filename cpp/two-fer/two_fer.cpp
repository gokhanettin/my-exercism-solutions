#include "two_fer.h"

namespace two_fer
{

std::string two_fer(const std::string& name)
{
    std::string s{"One for "};
    s.append(name);
    s.append(", one for me.");
    return s;
}

} // namespace two_fer

