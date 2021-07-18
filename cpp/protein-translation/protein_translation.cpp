#include "protein_translation.h"
#include <unordered_map>

namespace protein_translation {

std::vector<std::string> proteins(const std::string& seq)
{
    static const std::unordered_map<std::string, std::string> hash {
        {"AUG", "Methionine"   },
        {"UUU", "Phenylalanine"},
        {"UUC", "Phenylalanine"},
        {"UUA", "Leucine"      },
        {"UUG", "Leucine"      },
        {"UCU", "Serine"       },
        {"UCC", "Serine"       },
        {"UCA", "Serine"       },
        {"UCG", "Serine"       },
        {"UAU", "Tyrosine"     },
        {"UAC", "Tyrosine"     },
        {"UGU", "Cysteine"     },
        {"UGC", "Cysteine"     },
        {"UGG", "Tryptophan"   },
        {"UAA", ""             },
        {"UAG", ""             },
        {"UGA", ""             },
    };

    std::vector<std::string> vec;
    for (size_t i = 0;  i < seq.size(); i += 3) {
        const std::string codon{seq.substr(i, 3)};
        if (hash.find(codon) != end(hash)) {
            if (hash.at(codon).empty()) {
                break;
            }
            vec.push_back(hash.at(codon));
        }
    }
    return vec;
}

}  // namespace protein_translation
