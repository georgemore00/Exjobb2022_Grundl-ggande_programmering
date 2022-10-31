package main

import (
	"fmt"
	"math"
	"strconv"
)

func main() {
	option := ""
	var values [10]int = [10]int{}
	var size int = 0

	fmt.Println("Measurement tool 2.0")

	for option != "q" {

		fmt.Print("VECRQ? ")
		fmt.Scan(&option)

		switch option {
		case "v":
			fmt.Println(view(values, size))
		case "e":
			enterMeasurements(&values, &size)
		case "c":
			compute(values, size)
		case "r":
			values = [10]int{}
			fmt.Println("Reseted values")
		case "q":
			fmt.Println("Exit Measurement tool")
			break
		default:
			fmt.Println(option)
			fmt.Println("Invalid input, try again.")
		}
	}
}

func view(arr [10]int, size int) string {
	if size == 0 {
		return "No measurements"
	} else {
		s := "["
		for i := 0; i < size; i++ {
			s += strconv.FormatInt(int64(arr[i]), 10) + " "
		}
		s += "]"
		return s
	}
}

func enterMeasurements(arr *[10]int, size *int) {
	measurement := ""

	for i := *size; i < 10; i++ {
		fmt.Printf("Enter measurement #%v (or q to quit):", *size+1)
		fmt.Scan(&measurement)
		if measurement == "q" {
			break
		}

		intMeasurement, err := strconv.ParseInt(measurement, 10, 32)
		if err != nil {
			panic(err)
		}
		arr[i] = int(intMeasurement)
		*size += 1
	}
}

func compute(arr [10]int, size int) {
	if size == 0 {
		fmt.Println("Measurements empty.")
		return
	}
	fmt.Println("Max value: ", findMax(arr, size))
	fmt.Println("Min value: ", findMin(arr, size))
	fmt.Printf("Avr value: %.02f\n", findAverage(arr, size))
	fmt.Println(view(calcNormalizedValues(arr, size), size))
}

func findMin(arr [10]int, size int) int {
	if size == 0 {
		return 0
	}

	var min int = arr[0]
	for i := 0; i < size-1; i++ {
		if arr[i+1] < min {
			min = arr[i+1]
		}
	}
	return min
}

func findMax(arr [10]int, size int) int {
	if size == 0 {
		return 0
	}

	var max int = arr[0]
	for i := 0; i < size-1; i++ {
		if arr[i+1] > max {
			max = arr[i+1]
		}
	}
	return max
}

func findAverage(arr [10]int, size int) float64 {
	if size == 0 {
		return 0
	}

	var total float64 = 0.0
	for i := 0; i < size; i++ {
		total += float64(arr[i])
	}
	return total / float64(size)
}

func calcNormalizedValues(arr [10]int, size int) [10]int {
	var average float64 = findAverage(arr, size)
	var normalizedValues [10]int = [10]int{}

	for i := 0; i < size; i++ {
		var val int = arr[i] - int(math.Round(average))
		normalizedValues[i] = val
	}
	return normalizedValues
}
