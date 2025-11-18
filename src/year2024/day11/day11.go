// [Day 11: Plutonian Pebbles](https://adventofcode.com/2024/day/11)

package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
	"time"
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

func main() {
	filename := "input.txt"
	elapsed := false

	for i := 1; i < len(os.Args); i++ {
		arg := os.Args[i]
		if arg == "--elapsed" {
			elapsed = true
		} else if !strings.HasPrefix(arg, "-") {
			filename = arg
		}
	}

	data, err := os.ReadFile(filename)
	if err != nil {
		log.Fatal(err)
	}

	if elapsed {
		start := time.Now()
		result1, result2 := solve(data, 25), solve(data, 75)
		duration := time.Since(start)

		fmt.Println(result1)
		fmt.Println(result2)
		fmt.Printf("elapsed: %f ms\n", duration.Seconds()*1000.0)
	} else {
		result1, result2 := solve(data, 25), solve(data, 75)

		fmt.Println(result1)
		fmt.Println(result2)
	}
}
