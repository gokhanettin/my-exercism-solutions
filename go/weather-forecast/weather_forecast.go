// Package weather implements a weather forecasting function.
package weather

// CurrentCondition represents the current weather condtion.
var CurrentCondition string

// CurrentLocation represents the current location for the reported weather
// condition.
var CurrentLocation string

// Forecast sets CurrentLocation and CurrentCondition by the given inputs and
// return a string that tells the current weather condition for the given city.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
