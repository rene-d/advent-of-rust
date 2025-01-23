// [Day 7: Bridge Repair](https://adventofcode.com/2024/day/7)

package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func checkEquationTwoOperators(testValue int, values []int) bool {
	for i := 0; i < (1 << (len(values) - 1)); i++ {
		result := values[0]

		j, r := i, 0
		for _, value := range values[1:] {
			j, r = j/2, j%2
			if r == 0 {
				result += value
			} else {
				result *= value
			}
			if result > testValue {
				break
			}
		}
		if result == testValue {
			return true
		}
	}
	return false
}

func checkEquationThreeOperators(testValue int, values []int) bool {
	p10 := make([]int, len(values)-1)
	for idx, value := range values[1:] {
		p := 1
		v := value
		for v != 0 {
			p *= 10
			v /= 10
		}
		p10[idx] = p
	}

	for i := 0; i < intPow(3, len(values)-1); i++ {
		result := values[0]
		j, r := i, 0
		for idx, value := range values[1:] {
			j, r = j/3, j%3
			if r == 0 {
				result += value
			} else if r == 1 {
				result *= value
			} else {
				result = result*p10[idx] + value
			}
			if result > testValue {
				break
			}
		}
		if result == testValue {
			return true
		}
	}
	return false
}

func intPow(base, exp int) int {
	result := 1
	for exp > 0 {
		result *= base
		exp--
	}
	return result
}

func main() {

	inputFile := "input.txt"
	if len(os.Args) >= 2 {
		inputFile = os.Args[1]
	}

	file, err := os.Open(inputFile)
	if err != nil {
		fmt.Println("Error opening file:", err)
		return
	}
	defer file.Close()

	calibration_part1 := 0
	calibration_part2 := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Split(line, ":")
		testValue, err := strconv.Atoi(parts[0])
		if err != nil {
			fmt.Println("Error parsing test value:", err)
			continue
		}

		valuesStr := strings.Fields(parts[1])
		values := make([]int, len(valuesStr))
		for i, v := range valuesStr {
			value, err := strconv.Atoi(v)
			if err != nil {
				fmt.Println("Error parsing value:", err)
				continue
			}
			values[i] = value
		}

		if checkEquationTwoOperators(testValue, values) {
			calibration_part1 += testValue
		}

		if checkEquationThreeOperators(testValue, values) {
			calibration_part2 += testValue
		}

	}

	fmt.Println(calibration_part1)
	fmt.Println(calibration_part2)
}
