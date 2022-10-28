// Package weather provides tools for weather related utilities.
package weather

// CurrentCondition represents weather condition. 
var CurrentCondition string

// CurrentLocation represents the current city.
var CurrentLocation string

// Forecast returns a string in a human readable format for the current location with the current condition.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}