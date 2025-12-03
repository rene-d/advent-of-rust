// [Day 3: Lobby](https://adventofcode.com/2025/day/3)

package main

import (
	"fmt"
	"log"
	"os"
	"strings"
	"time"
)

func largestJoltageN(bank string, n int) uint64 {

	batteries := make([]byte, 0, len(bank))
	remove := len(bank) - n

	for _, battery := range []byte(bank) {
		for remove > 0 && len(batteries) > 0 && batteries[len(batteries)-1] < battery {
			batteries = batteries[:len(batteries)-1]
			remove--
		}
		batteries = append(batteries, battery)
	}

	var joltage uint64
	for i := 0; i < n && i < len(batteries); i++ {
		joltage = joltage*10 + uint64(batteries[i]-'0')
	}
	return joltage
}

func solve(data []byte) (uint64, uint64) {
	var part1, part2 uint64
	banks := strings.Split(strings.TrimSpace(string(data)), "\n")

	for _, bank := range banks {
		part1 += largestJoltageN(bank, 2)
		part2 += largestJoltageN(bank, 12)
	}

	return part1, part2
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
		result1, result2 := solve(data)
		duration := time.Since(start)

		fmt.Println(result1)
		fmt.Println(result2)
		fmt.Printf("elapsed: %f ms\n", duration.Seconds()*1000.0)
	} else {
		result1, result2 := solve(data)

		fmt.Println(result1)
		fmt.Println(result2)
	}
}
