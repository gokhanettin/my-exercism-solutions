package techpalace

import (
	"fmt"
	"strings"
)

// WelcomeMessage returns a welcome message for the customer.
func WelcomeMessage(customer string) string {
	message := "Welcome to the Tech Palace"
	name := strings.ToUpper(customer)
	return fmt.Sprintf("%v, %v", message, name)
}

// AddBorder adds a border to a welcome message.
func AddBorder(welcomeMsg string, numStarsPerLine int) string {
	stars := strings.Repeat("*", numStarsPerLine)
	return fmt.Sprintf("%v\n%v\n%v", stars, welcomeMsg, stars)
}

// CleanupMessage cleans up an old marketing message.
func CleanupMessage(oldMsg string) string {
	s := strings.ReplaceAll(oldMsg, "*", "")
	return strings.TrimSpace(s)
}
