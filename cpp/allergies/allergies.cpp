#include "allergies.h"
#include <vector>

namespace allergies {

allergy_test::allergy_test(int score)
{

    const static std::vector<std::string> items {
        "eggs",
        "peanuts",
        "shellfish",
        "strawberries",
        "tomatoes",
        "chocolate",
        "pollen",
        "cats",
    };

    for (size_t i = 0; i < items.size(); ++i) {
        if (score & (1 << i)) {
            allergies_.insert(items[i]);
        }
    }

}

bool allergy_test::is_allergic_to(const std::string& item) const
{
    return allergies_.find(item) != end(allergies_);
}

std::unordered_set<std::string> allergy_test::get_allergies() const
{
    return allergies_;
}

}  // namespace allergies
