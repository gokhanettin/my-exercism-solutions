package cars

// DefaultCarProductionPerHour when speed is 1
const DefaultCarProductionPerHour = 221

// SuccessRate is used to calculate the ratio of an item being created without
// error for a given speed
func SuccessRate(speed int) float64 {
	var rate float64
	switch {
	case speed == 0:
		rate = 0.
	case speed >= 1 && speed <= 4:
		rate = 1.
	case speed >= 5 && speed <= 8:
		rate = 0.9
	case speed >= 9 && speed <= 77:
		rate = 0.77
	default:
		panic("Unexpected speed!")
	}
	return rate
}

// CalculateProductionRatePerHour for the assembly line, taking into account
// its success rate
func CalculateProductionRatePerHour(speed int) float64 {
	return float64(speed) * SuccessRate(speed) * DefaultCarProductionPerHour
}

// CalculateProductionRatePerMinute describes how many working items are
// produced by the assembly line every minute
func CalculateProductionRatePerMinute(speed int) int {
	return int(CalculateProductionRatePerHour(speed) / 60.)
}

// CalculateLimitedProductionRatePerHour describes how many working items are
// produced per hour with an upper limit on how many can be produced per hour
func CalculateLimitedProductionRatePerHour(speed int, limit float64) float64 {
	rate := CalculateProductionRatePerHour(speed)
	if rate <= limit {
		return rate
	}
	return limit
}
