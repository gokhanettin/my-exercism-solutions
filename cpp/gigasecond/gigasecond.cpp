#include "gigasecond.h"

namespace gigasecond {

constexpr unsigned long GIGASECOND = 1'000'000'000;

ptime advance(const ptime& time)
{
    return time + boost::posix_time::seconds(GIGASECOND);
}

}  // namespace gigasecond
