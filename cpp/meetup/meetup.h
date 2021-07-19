#if !defined(MEETUP_H)
#define MEETUP_H

#include <boost/date_time/gregorian/gregorian.hpp>

namespace meetup {

#define DECLARE_DAY_METHODS(param) \
    date param##teenth() const; \
    date first_##param##day() const; \
    date second_##param##day() const; \
    date third_##param##day() const; \
    date fourth_##param##day() const; \
    date last_##param##day() const;

class scheduler {
    public:
        using date = boost::gregorian::date;
        using greg_month = boost::gregorian::greg_month;
        using greg_year = boost::gregorian::greg_year;

        scheduler(greg_month month, greg_year year);
        DECLARE_DAY_METHODS(mon)
        DECLARE_DAY_METHODS(tues)
        DECLARE_DAY_METHODS(wednes)
        DECLARE_DAY_METHODS(thurs)
        DECLARE_DAY_METHODS(fri)
        DECLARE_DAY_METHODS(satur)
        DECLARE_DAY_METHODS(sun)
    private:
        greg_month month_;
        greg_year  year_;
};

}  // namespace meetup

#endif // MEETUP_H