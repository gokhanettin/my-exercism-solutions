#include "rna_transcription.h"
#include <algorithm>

namespace rna_transcription {

std::string to_rna(const std::string& dna)
{
    std::string rna{dna};
    std::transform(begin(rna), end(rna), begin(rna),
     static_cast<char (*)(char)>(to_rna));

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
