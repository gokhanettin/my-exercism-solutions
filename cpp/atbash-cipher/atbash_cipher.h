#if !defined(ATBASH_CIPHER_H)
#define ATBASH_CIPHER_H

#include <string>

namespace atbash_cipher {
std::string encode(const std::string& s);
std::string decode(const std::string& s);
}  // namespace atbash_cipher

#endif  // ATBASH_CIPHER_H