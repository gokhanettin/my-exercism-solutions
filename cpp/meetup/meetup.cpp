#include "meetup.h"

namespace meetup {

namespace {
using boost::gregorian::greg_day;
using boost::gregorian::Monday;
using boost::gregorian::Tuesday;
using boost::gregorian::Wednesday;
using boost::gregorian::Thursday;
using boost::gregorian::Friday;
using boost::gregorian::Saturday;
using boost::gregorian::Sunday;
using boost::date_time::first_kday_after;
using boost::date_time::last_kday_of_month;
using boost::date_time::nth_kday_of_month;

#define _DEFINE_FIRST_TO_FOURTH_DAY_METHODS(param, param1, param2) \
scheduler::date scheduler::param##_##param1##day() const \
    { return nth_kday_of_month<date>{nth_kday_of_month<date>::param, param2, month_}.get_date(year_); } \

#define DEFINE_DAY_METHODS(param1, param2) \
_DEFINE_FIRST_TO_FOURTH_DAY_METHODS(first, param1, param2) \
_DEFINE_FIRST_TO_FOURTH_DAY_METHODS(second, param1, param2) \
_DEFINE_FIRST_TO_FOURTH_DAY_METHODS(third, param1, param2) \
_DEFINE_FIRST_TO_FOURTH_DAY_METHODS(fourth, param1, param2) \
scheduler::date scheduler::param1##teenth() const \
    { return first_kday_after<date>{param2}.get_date(date{year_, month_, 12}); } \
scheduler::date scheduler::last_##param1##day() const \
{ return last_kday_of_month<date>{param2, month_}.get_date(year_); }

}

scheduler::scheduler(greg_month month, greg_year year)
    :month_{month}, year_{year} {}

DEFINE_DAY_METHODS(mon, Monday)
DEFINE_DAY_METHODS(tues, Tuesday)
DEFINE_DAY_METHODS(wednes, Wednesday)
DEFINE_DAY_METHODS(thurs, Thursday)
DEFINE_DAY_METHODS(fri, Friday)
DEFINE_DAY_METHODS(satur, Saturday)
DEFINE_DAY_METHODS(sun, Sunday)

}  // namespace meetup
