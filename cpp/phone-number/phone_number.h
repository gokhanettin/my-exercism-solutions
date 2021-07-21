#if !defined(PHONE_NUMBER_H)
#define PHONE_NUMBER_H

#include <string>

namespace phone_number {
class phone_number {
public:
    explicit phone_number(const std::string& s);
    explicit operator std::string() const;

    std::string number() const;
    std::string area_code() const;

private:
    void format(std::string& s);
    std::string phone_;
};
} // namespace phone_number

#endif // PHONE_NUMBER_H