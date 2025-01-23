// [Day 3: Mull It Over](https://adventofcode.com/2024/day/3)

package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
)

// solve computes valid mul() operations.
// If part1 is false, obey to the do()/don't() statements.
func solve(data string, part1 bool) int64 {
	var enabled bool = true
	var totalSum int64

	re := regexp.MustCompile(`mul\((\d+),(\d+)\)|do\(\)|don't\(\)`)

	for _, m := range re.FindAllStringSubmatch(data, -1) {
		if m[0] == "do()" {
			enabled = true
		} else if m[0] == "don't()" {
			enabled = false
		} else if enabled || part1 {
			x, _ := strconv.ParseInt(m[1], 10, 0)
			y, _ := strconv.ParseInt(m[2], 10, 0)
			totalSum += x * y
		}
	}

	return totalSum
}

func main() {

	inputFile := "input.txt"
	if len(os.Args) >= 2 {
		inputFile = os.Args[1]
	}

	data, err := os.ReadFile(inputFile)
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	fmt.Println(solve(string(data), true))
	fmt.Println(solve(string(data), false))
}
