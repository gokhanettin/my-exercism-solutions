#include "rna_transcription.h"

namespace rna_transcription {

std::string to_rna(const std::string& dna)
{
    std::string rna{dna};
    for (auto it{begin(rna)}; it != end(rna); ++it) {
        *it = to_rna(*it);
    }
    return rna;
}

char to_rna(char nucleotide)
{
    switch (nucleotide) {
        case 'G': return 'C';
        case 'C': return 'G';
        case 'T': return 'A';
        case 'A': return 'U';
    }

    return 'X';
}


}  // namespace rna_transcription
