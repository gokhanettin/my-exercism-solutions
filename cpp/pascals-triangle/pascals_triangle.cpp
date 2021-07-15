#include "pascals_triangle.h"

namespace pascals_triangle {

    std::vector<std::vector<int>> generate_rows(int nrows)
    {
        if (nrows <= 0) {
            return {};
        } else {
            std::vector<std::vector<int>> vectors {{1}};
            vectors.reserve(nrows);

            for (int row{1}; row < nrows; ++row) {
                std::vector<int> curr{1};
                curr.reserve(row);
                const auto prev = vectors.back();
                for (size_t i{1}; i < prev.size(); ++i) {
                    curr.push_back(prev[i - 1] + prev[i]);
                }
                curr.push_back(1);
                vectors.emplace_back(curr);
            }
            return vectors;
        }
    }

}  // namespace pascals_triangle
