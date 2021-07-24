#if !defined(CRYPTO_SQUARE_H)
#define CRYPTO_SQUARE_H

#include <string>
#include <vector>

namespace crypto_square {
class cipher {
public:
    explicit cipher(const std::string& text);
    explicit cipher(std::string&& text);

    std::string normalize_plain_text() const;
    std::vector<std::string> plain_text_segments() const;
    std::string cipher_text() const;
    std::string normalized_cipher_text() const;
private:
    std::string text_;
};
} // namespace crypto_square

#endif // CRYPTO_SQUARE_H