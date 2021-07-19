#if !defined(CLOCK_H)
#define CLOCK_H

#include <string>

namespace date_independent {
class clock {
    public:
        clock plus(int amount) const;
        explicit operator std::string() const;

        static clock at(int hour, int minute);

        friend bool operator==(const clock& lhs, const clock& rhs);
        friend bool operator!=(const clock& lhs, const clock& rhs);
    private:
        void normalize();
        clock(int hour, int minute);

        int hour_;
        int minute_;
};
}  // namespace date_independent

#endif // CLOCK_H