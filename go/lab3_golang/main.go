package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Medicine struct {
	name      string
	sizes     [10]int
	balances  [10]int
	nrOfSizes int
}

func main() {
	const MAX_SIZE int = 10000
	var nrOfMedicines int = 0
	var medicines [10000]Medicine = [MAX_SIZE]Medicine{}
	var nrOfFoundInSearch int = 0
	var foundIndex int = -1
	filePath := ""

	fmt.Println("Apoteks verktyg 2.0")
	fmt.Println("Vanligen ange filnamn: ")
	fmt.Scan(&filePath)
	readFromFile(filePath, medicines[:], &nrOfMedicines, MAX_SIZE)

	option := ""
	for option != "q" {

		fmt.Print("RPSD?")
		fmt.Scan(&option)

		switch option {
		case "r":
			registerMedicine(medicines[:], &nrOfMedicines)
		case "p":
			fmt.Println(printMedicines(medicines[:], nrOfMedicines))
		case "s":
			var found []Medicine = search(medicines[:], nrOfMedicines, &nrOfFoundInSearch, &foundIndex)
			fmt.Println(printMedicines(found, nrOfFoundInSearch))
		case "d":
			deleteMedicine(medicines[:], &nrOfMedicines, &nrOfFoundInSearch, &foundIndex)
		case "q":
			writeToFile(filePath, medicines[:], nrOfMedicines)
			fmt.Println("Exiting Pharmacy tool 2.0")
			break
		default:
			fmt.Println(option)
			fmt.Println("Invalid inmatning, försök igen.")
		}
	}
}

func readFromFile(filePath string, medicines []Medicine, nrOfMedicines *int, MAX_SIZE int) {
	file, err := os.Open(filePath)
	if err != nil {
		fmt.Printf("Kunde inte hitta fil: %s, den kommer skapas vid programslut. \n", filePath)
		return
	}

	var scanner *bufio.Scanner = bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)
	scanner.Scan()
	firstLine, err := strconv.Atoi(scanner.Text())

	if firstLine > MAX_SIZE {
		fmt.Println("För många läkemedel i filen, kan inte läsa in.")
		return
	}
	*nrOfMedicines = firstLine

	for i := 0; i < *nrOfMedicines; i++ {
		scanner.Scan()
		line := scanner.Text()
		medicines[i] = parseLineToMedicine(line)
	}
	file.Close()
}

func parseLineToMedicine(line string) Medicine {
	var columns []string = strings.Split(line, " ")
	var m Medicine = Medicine{}

	name := columns[0]
	var sizes []string = strings.Split(columns[1], ",")
	var balances []string = strings.Split(columns[2], ",")
	nrOfSizes, err := strconv.Atoi(columns[3])

	m.name = name
	for i := 0; i < 10; i++ {
		m.sizes[i], err = strconv.Atoi(sizes[i])
	}
	for i := 0; i < 10; i++ {
		m.balances[i], err = strconv.Atoi(balances[i])
	}
	if err != nil {
		fmt.Println("malformerad text fil")
	}
	return Medicine{name: name, sizes: m.sizes, balances: m.balances, nrOfSizes: nrOfSizes}
}

func writeToFile(filePath string, medicines []Medicine, nrOfMedicines int) {
	file, err := os.Create(filePath)
	if err != nil {
		return
	}
	defer file.Close()

	file.WriteString(strconv.Itoa(nrOfMedicines) + "\n")

	for i := 0; i < nrOfMedicines; i++ {
		file.WriteString(medicines[i].name + " ")

		for j := 0; j < 10; j++ {
			s := strconv.Itoa(medicines[i].sizes[j])
			file.WriteString(s + ",")
		}
		file.WriteString(" ")

		for j := 0; j < 10; j++ {
			s := strconv.Itoa(medicines[i].balances[j])
			file.WriteString(s + ",")
		}
		file.WriteString(" ")

		s := strconv.Itoa(medicines[i].nrOfSizes)
		file.WriteString(s)
		file.WriteString("\n")
	}
}

func medicineAlreadyExists(medicines []Medicine, nrOfMedicines int, name string) bool {
	for i := 0; i < nrOfMedicines; i++ {
		if medicines[i].name == name {
			return true
		}
	}
	return false
}

func registerMedicine(medicines []Medicine, nrOfMedicines *int) {
	if *nrOfMedicines == len(medicines) {
		fmt.Println("Finns inget mer utrymme för fler mediciner.")
		return
	}

	name := ""
	var size int = -1

	fmt.Println("Registera lakemedel")
	for {
		fmt.Print("Ange namn: ")
		fmt.Scan(&name)
		if !medicineAlreadyExists(medicines, *nrOfMedicines, name) {
			break
		}
		fmt.Printf("Medicin med namn %s existerar redan, prova igen \n", name)
	}

	var m Medicine = Medicine{name: name}

	for i := 0; i < 10; i++ {
		fmt.Println("Ange storlek (0 för att avsluta): ")
		fmt.Scan(&size)
		if size == 0 {
			break
		}
		m.sizes[i] = size
		m.nrOfSizes++
	}

	*&medicines[*nrOfMedicines] = m
	*nrOfMedicines++
}

func deleteMedicine(medicines []Medicine, nrOfMedicines *int, nrOfFoundInSearch *int, foundIndex *int) {
	answer := ""
	*nrOfFoundInSearch = 0
	for *nrOfFoundInSearch != 1 {
		var found []Medicine = search(medicines[:], *nrOfMedicines, nrOfFoundInSearch, foundIndex)
		fmt.Println(printMedicines(found, *nrOfFoundInSearch))
		if *nrOfFoundInSearch == 1 {
			fmt.Printf("Vill du avregistrera %s (j/n)?", medicines[*foundIndex].name)
			fmt.Scan(&answer)
			if answer == "j" {
				medicines = append(medicines[:*foundIndex], medicines[*foundIndex+1:]...)
				*nrOfMedicines--
			}
		} else {
			fmt.Println("Du fick inte endast ett alternativ. Vanligen gor en ny sokning.")
		}
	}
}

func printMedicines(medicines []Medicine, nrOfMedicines int) string {
	s := fmt.Sprintln("Lakemedel \t Storlekar \t Saldo")
	s += fmt.Sprintln("__________________________________________________")

	for i := 0; i < nrOfMedicines; i++ {
		s += fmt.Sprint(medicines[i].name + "\t\t")

		for j := 0; j < medicines[i].nrOfSizes; j++ {
			s += fmt.Sprintf("%v,", medicines[i].sizes[j])
		}

		s += fmt.Sprint("\t\t")

		for j := 0; j < medicines[i].nrOfSizes; j++ {
			s += fmt.Sprintf("%v,", medicines[i].balances[j])
		}

		s += fmt.Sprintln()
	}

	return s
}

func search(medicines []Medicine, nrOfMedicines int, nrOfFoundInSearch *int, foundIndex *int) []Medicine {
	name := ""
	//Tvungen att använda make funktion då found arrayens storlek inte går att specificera
	// med nrOfMedicines då den inte är en const
	var found []Medicine = make([]Medicine, nrOfMedicines)
	*nrOfFoundInSearch = 0

	fmt.Println("Soka lakemedel")
	fmt.Print("Ange namn: ")
	fmt.Scan(&name)

	for i := 0; i < nrOfMedicines; i++ {
		if strings.Contains(medicines[i].name, name) {
			found[*nrOfFoundInSearch] = medicines[i]
			*nrOfFoundInSearch += 1
			*foundIndex = i
		}
	}
	return found
}
