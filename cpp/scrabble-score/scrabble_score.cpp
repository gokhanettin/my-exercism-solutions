#include "scrabble_score.h"
#include <unordered_map>
#include <vector>
#include <algorithm>
#include <numeric>

namespace scrabble_score {
    int score(const std::string& s)
    {
        const std::unordered_map<int, int> table {
            {'A', 1}, {'E', 1}, {'I', 1}, {'O', 1}, {'U', 1}, {'L', 1},
            {'N', 1}, {'R', 1}, {'S', 1}, {'T', 1}, {'D', 2}, {'G', 2},
            {'B', 3}, {'C', 3}, {'M', 3}, {'P', 3},
            {'F', 4}, {'H', 4}, {'V', 4}, {'W', 4}, {'Y', 4},
            {'K', 5},
            {'J', 8}, {'X', 8},
            {'Q', 10}, {'Z', 10},
        };

        std::vector<int> scores;
        std::transform(begin(s), end(s), std::back_inserter(scores),
            [&table](int c) { return table.at(std::toupper(c)); });
        return std::accumulate(begin(scores), end(scores), 0);

        // int score = 0;
        // for (auto c : s) {
        //     c = std::toupper(c);
        //     score += table.at(c);
        // }
        // return score;
    }
}  // namespace scrabble_score
