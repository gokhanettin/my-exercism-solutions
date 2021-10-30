package partyrobot

import "fmt"

// Welcome greets a person by name.
func Welcome(name string) string {
	welcome := "Welcome to my party"
	return fmt.Sprintf("%v, %v!", welcome, name)
}

// HappyBirthday wishes happy birthday to the birthday person and exclaims their age.
func HappyBirthday(name string, age int) string {
	return fmt.Sprintf("Happy birthday %v! You are now %v years old!", name, age)
}

// AssignTable assigns a table to each guest.
func AssignTable(name string, table int, neighbor, direction string, distance float64) string {
	format := "Welcome to my party, %s!\n" +
		"You have been assigned to table %03d. Your table is %s, exactly %.1f meters from here.\n" +
		"You will be sitting next to %s."
	return fmt.Sprintf(format, name, table, direction, distance, neighbor)
}
