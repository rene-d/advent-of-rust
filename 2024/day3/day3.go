// [Day 3: Mull It Over](https://adventofcode.com/2024/day/3)

package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"regexp"
	"strconv"
)

// solve computes valid mul() operations.
// If part1 is false, obey to the do()/don't() statements.
func solve(data string, part1 bool) int {
	var enabled bool = true
	var totalSum int
	i := 0

	re := regexp.MustCompile(`^mul\((\d+),(\d+)\)`)

	for i < len(data) {
		if i+4 <= len(data) && data[i:i+4] == "do()" {
			enabled = true
			i += 4

		} else if i+7 <= len(data) && data[i:i+7] == "don't()" {
			enabled = false
			i += 7

		} else if i+4 <= len(data) && data[i:i+4] == "mul(" {
			matches := re.FindStringSubmatch(data[i:])
			if matches != nil {
				if enabled || part1 {
					x, _ := strconv.Atoi(matches[1])
					y, _ := strconv.Atoi(matches[2])
					totalSum += x * y
				}
				i += len(matches[0])
			} else {
				i += 4
			}

		} else {
			i++
		}
	}

	return totalSum
}

func main() {

	inputFile := "input.txt"
	if len(os.Args) >= 2 {
		inputFile = os.Args[1]
	}

	data, err := ioutil.ReadFile(inputFile)
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	fmt.Println(solve(string(data), true))
	fmt.Println(solve(string(data), false))
}
