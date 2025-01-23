// [Day 1: Historian Hysteria](https://adventofcode.com/2024/day/1)

package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {

	inputFile := "input.txt"
	if len(os.Args) >= 2 {
		inputFile = os.Args[1]
	}

	file, err := os.Open(inputFile)
	if err != nil {
		fmt.Println("error:", err)
		return
	}
	defer file.Close()

	var left, right []int

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Fields(line)
		a, _ := strconv.Atoi(parts[0])
		b, _ := strconv.Atoi(parts[1])
		left = append(left, a)
		right = append(right, b)
	}

	if err := scanner.Err(); err != nil {
		fmt.Println("error:", err)
		return
	}

	// sort the lists
	sort.Ints(left)
	sort.Ints(right)

	// part 1
	part1 := 0
	for i := 0; i < len(left); i++ {
		part1 += abs(left[i] - right[i])
	}
	fmt.Println(part1)

	// part 2
	rightCount := make(map[int]int)
	for _, v := range right {
		rightCount[v]++
	}

	part2 := 0
	for _, a := range left {
		part2 += a * rightCount[a]
	}
	fmt.Println(part2)
}

// abs returns the absolute value of x.
func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}
