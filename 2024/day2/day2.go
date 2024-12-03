// [Day 2: Red-Nosed Reports](https://adventofcode.com/2024/day/2)

package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

// isSafe returns true if the all levels are safe:
//   - The levels are either all increasing or all decreasing.
//   - Any two adjacent levels differ by at least one and at most three.
func isSafe(v []int) bool {

	decreasing, increasing := true, true

	for i := 0; i < len(v)-1; i++ {
		if !(1 <= v[i]-v[i+1] && v[i]-v[i+1] <= 3) {
			decreasing = false
		}

		if !(1 <= v[i+1]-v[i] && v[i+1]-v[i] <= 3) {
			increasing = false
		}
	}

	return decreasing || increasing
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

	// Read input file
	scanner := bufio.NewScanner(file)
	var lines [][]int
	for scanner.Scan() {
		line := scanner.Text()
		numStrings := strings.Split(line, " ")

		var nums []int
		for _, str := range numStrings {
			num, _ := strconv.Atoi(str)
			nums = append(nums, num)
		}

		lines = append(lines, nums)
	}

	// Part 1
	part1 := 0
	for _, nums := range lines {
		if isSafe(nums) {
			part1 += 1
		}
	}
	fmt.Println(part1)

	// Part 2
	part2 := 0
	for _, nums := range lines {
		if isSafe(nums) {
			part2++
		} else {
			for i := 0; i < len(nums); i++ {
				temp := append([]int(nil), nums[:i]...)
				temp = append(temp, nums[i+1:]...)
				if isSafe(temp) {
					part2++
					break
				}
			}
		}
	}
	fmt.Println(part2)
}
