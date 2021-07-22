#include "complex_numbers.h"
#include <cmath>

namespace complex_numbers {
Complex::Complex()
    : Complex{0.0, 0.0}
{
}

Complex::Complex(double re, double im)
    : re_{re}
    , im_{im}
{
}

double Complex::real() const
{
    return re_;
}

double Complex::imag() const
{
    return im_;
}

Complex Complex::conj() const
{
    return Complex{re_, -im_};
}

Complex Complex::exp() const
{
    double exp = std::exp(re_);
    return {exp * std::cos(im_), exp * std::sin(im_)};
}

double Complex::abs() const
{
    return std::sqrt(re_ * re_ + im_ * im_);
}

Complex operator+(const Complex& lhs, const Complex& rhs)
{
    return {lhs.re_ + rhs.re_, lhs.im_ + rhs.im_};
}

Complex operator-(const Complex& lhs, const Complex& rhs)
{
    return {lhs.re_ - rhs.re_, lhs.im_ - rhs.im_};
}

Complex operator*(const Complex& lhs, const Complex& rhs)
{
    double re = lhs.re_ * rhs.re_ - lhs.im_ * rhs.im_;
    double im = lhs.im_ * rhs.re_ + lhs.re_ * rhs.im_;
    return {re, im};
}

Complex operator/(const Complex& lhs, const Complex& rhs)
{
    Complex numerator = lhs * rhs.conj();
    double denominator = std::pow(rhs.abs(), 2);

    return {numerator.re_ / denominator, numerator.im_ / denominator};
}
} // namespace complex_numbers
