package lasagna

// PreparationTime returns the total time required for each layer.
func PreparationTime(layers []string, average int) int {
	if average == 0 {
		average = 2
	}
	return average * len(layers)
}

// Quantities returns the required  total amount of noodles and sauce.
func Quantities(layers []string) (int, float64) {
	var sauce int
	var noodles int
	for _, layer := range layers {
		if layer == "sauce" {
			sauce++
		} else if layer == "noodles" {
			noodles++
		}
	}
	return noodles * 50, float64(sauce) * 0.2
}

// AddSecretIngredient adds the last secret ingredient from friend's list.
func AddSecretIngredient(friendList, myList []string) []string {
	return append(myList, friendList[len(friendList)-1])
}

// ScaleRecipe scales the recipe for the number of portions we want to cook.
func ScaleRecipe(amounts []float64, portion int) []float64 {
	var scaled []float64
	for _, amount := range amounts {
		scaled = append(scaled, (amount/2.0)*float64(portion))
	}
	return scaled
}
