#include "crypto_square.h"
#include <algorithm>
#include <cmath>
#include <utility>

namespace crypto_square {
cipher::cipher(const std::string& text)
    : text_{text}
{
}

cipher::cipher(std::string&& text)
    : text_{std::move(text)}
{
}

std::string cipher::normalize_plain_text() const
{
    std::string text{text_};
    text.erase(std::remove_if(begin(text), end(text),
                   [](auto c) { return std::isspace(c) || std::ispunct(c); }),
        end(text));
    std::transform(begin(text), end(text), begin(text), ::tolower);
    return text;
}

std::vector<std::string> cipher::plain_text_segments() const
{
    std::string text{normalize_plain_text()};
    double sqrt = std::sqrt(text.size());
    int c = static_cast<int>(std::ceil(sqrt));
    int r = static_cast<int>(std::floor(sqrt));

    std::vector<std::string> result;
    result.reserve(r);
    for (size_t i = 0; i < text.size(); i += c) {
        result.push_back(text.substr(i, c));
    }
    return result;
}

std::string cipher::cipher_text() const
{
    std::string result{normalized_cipher_text()};
    result.erase(remove_if(begin(result), end(result), ::isspace), end(result));
    return result;
}

std::string cipher::normalized_cipher_text() const
{
    std::vector<std::string> segments{plain_text_segments()};
    std::string result;
    size_t col = 0;

    while (true) {
        size_t exhausted = 0;
        for (const auto& segment : segments) {
            if (col < segment.size()) {
                result.push_back(segment[col]);
            } else {
                ++exhausted;
            }
        }
        if (exhausted == 1) {
            result.push_back(' ');
        }
        if (exhausted != segments.size()) {
            result.push_back(' ');
        } else {
            if (!result.empty()) {
                result.pop_back();
            }
            break;
        }
        ++col;
    }
    return result;
}
} // namespace crypto_square
