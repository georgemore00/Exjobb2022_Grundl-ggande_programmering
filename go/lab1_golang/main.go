package main

import (
	"fmt"
	"log"
)

/*
SEKVENS FÖR ATT LYCKAS MED LANDNINGEN
0
0
0
0
0
0
100
100
100
100
0
0
30
*/

func main() {
	var time int32 = 0
	var height float64 = 250.0
	var velocity float64 = -25.0
	var throttle float64 = 0.0

	fmt.Println(getInitialMessage())

	for height > 0 {
		fmt.Printf("%v \t %0.2f \t %0.2f \t %v \n ", time, height, velocity, throttle)

		if _, err := fmt.Scan(&throttle); err != nil {
			log.Print("  Scan for throttle failed, due to ", err)
			return
		}
		if throttle < 0 || throttle > 100 {
			fmt.Println("Scan for throttle failed, input must be between 0-100.")
			return
		}

		height = calcNewHeight(height, velocity, throttle)
		velocity = calcNewVelocity(velocity, throttle)
		time++
	}

	if velocity >= -2.0 {
		fmt.Println("SUCCESS! Landed at", velocity, "m/s")
	} else {
		fmt.Println("FAILED! Crash landing at", velocity, "m/s")
	}
}

func calcNewHeight(currentHeight float64, currentVelocity float64, throttle float64) float64 {
	return currentHeight + currentVelocity + ((throttle*0.1 - 1.5) / 2)
}

func calcNewVelocity(currentVelocity float64, throttle float64) float64 {
	return currentVelocity + (throttle*0.1 - 1.5)
}

// Bad practice att concat strängar med + operator då dem är immutable
// Dock blev utskriften ful med multilines strängar
// Och självklart vill man inte ha en extremt lång sträng på en rad.
func getInitialMessage() string {
	return "Lunar decent challenge!\nYou will pilot a lunar decent the last 250m.\nEach turn represent 1-second decent time.\n" +
		"Set the throttle for each turn (0-100)\nTime Height Velocity Throttle?\n"
}
