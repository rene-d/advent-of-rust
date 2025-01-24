// [Day 11: Plutonian Pebbles](https://adventofcode.com/2024/day/11)

package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

// transform applies a the transformation of a stone according to the process of the puzzle statement.
func transform(stone int) []int {
	// engraved with the number 0 => 1
	if stone == 0 {
		return []int{1}
	}

	// engraved with a number that has an even number of digits => split it into two stones
	stoneStr := strconv.Itoa(stone)
	if len(stoneStr)%2 == 0 {
		mid := len(stoneStr) / 2
		left, _ := strconv.Atoi(stoneStr[:mid])
		right, _ := strconv.Atoi(stoneStr[mid:])
		return []int{left, right}
	}

	// otherwise, multiply by 2024
	return []int{stone * 2024}
}

// blink blinks all stones within the frequency map returning a new frequency map.
func blink(stoneCounts map[int]int) map[int]int {
	newStoneCounts := make(map[int]int)

	for stone, count := range stoneCounts {
		newStones := transform(stone)
		for _, newStone := range newStones {
			newStoneCounts[newStone] += count
		}
	}

	return newStoneCounts
}

// solve reads the input puzzle from data, computes the stone frequency map and blinks it numBlinks times.
// It returns the stone count.
func solve(data []byte, numBlinks int) int {

	strStones := strings.Fields(string(data))
	stoneCounts := make(map[int]int)
	for _, str := range strStones {
		stone, err := strconv.Atoi(str)
		if err != nil {
			return -1
		}
		stoneCounts[stone]++
	}

	for i := 0; i < numBlinks; i++ {
		stoneCounts = blink(stoneCounts)
	}

	totalStones := 0
	for _, count := range stoneCounts {
		totalStones += count
	}

	return totalStones
}

// main is the entry point of the program.
func main() {
	filename := "input.txt"
	if len(os.Args) >= 2 {
		filename = os.Args[1]
	}
	data, err := os.ReadFile(filename)
	if err != nil {
		log.Fatal(err)
	}

	// part 1
	fmt.Println(solve(data, 25))

	// part 2
	fmt.Println(solve(data, 75))
}
