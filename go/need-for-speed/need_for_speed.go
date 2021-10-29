package speed

// Car represents remote controlled car.
type Car struct {
	battery      int
	batteryDrain int
	speed        int // meter per second
	distance     int // meter
}

// NewCar creates a new remote controlled car with full battery and given specifications.
func NewCar(speed, batteryDrain int) Car {
	return Car{battery: 100, batteryDrain: batteryDrain, speed: speed}
}

// Track represents the race track distance in meter.
type Track struct {
	distance int // meter
}

// NewTrack created a new track
func NewTrack(distance int) Track {
	return Track{distance}
}

// Drive drives the car one time. If there is not enough battery to drive on more time,
// the car will not move.
func Drive(car Car) Car {
	if car.battery < car.batteryDrain {
		return car
	}
	car.battery -= car.batteryDrain
	car.distance += car.speed
	return car
}

// CanFinish checks if a car is able to finish a certain track.
func CanFinish(car Car, track Track) bool {
	for car.battery >= car.batteryDrain {
		car = Drive(car)
		if car.distance >= track.distance {
			return true
		}
	}
	return false
}
