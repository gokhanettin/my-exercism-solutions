#if !defined(COMPLEX_NUMBERS_H)
#define COMPLEX_NUMBERS_H

namespace complex_numbers {
class Complex {
public:
    Complex();
    Complex(double re, double im);

    double real() const;
    double imag() const;
    Complex conj() const;
    Complex exp() const;
    double abs() const;

    friend Complex operator+(const Complex& lhs, const Complex& rhs);
    friend Complex operator-(const Complex& lhs, const Complex& rhs);
    friend Complex operator*(const Complex& lhs, const Complex& rhs);
    friend Complex operator/(const Complex& lhs, const Complex& rhs);

private:
    double re_;
    double im_;
};
} // namespace complex_numbers

#endif // COMPLEX_NUMBERS_H
