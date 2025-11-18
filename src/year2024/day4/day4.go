// [Day 4: Ceres Search](https://adventofcode.com/2024/day/4)

package main

import (
	"fmt"
	"log"
	"os"
	"strings"
	"time"
)

func solve(data []byte) (int, int) {

	lines := strings.Split(strings.TrimSpace(string(data)), "\n")
	sy := len(lines)
	sx := len(lines[0])

	letters := func(x, y int) byte {
		if 0 <= x && x < sx && 0 <= y && y < sy {
			return lines[y][x]
		}
		return '.'
	}

	// Part 1
	part1 := 0
	for y := 0; y < sy; y++ {
		for x := 0; x < sx; x++ {
			if letters(x, y) == 'X' {
				// Check 8 directions for "MAS"
				if string([]byte{letters(x, y+1), letters(x, y+2), letters(x, y+3)}) == "MAS" {
					part1++
				}
				if string([]byte{letters(x+1, y), letters(x+2, y), letters(x+3, y)}) == "MAS" {
					part1++
				}
				if string([]byte{letters(x+1, y+1), letters(x+2, y+2), letters(x+3, y+3)}) == "MAS" {
					part1++
				}
				if string([]byte{letters(x+1, y-1), letters(x+2, y-2), letters(x+3, y-3)}) == "MAS" {
					part1++
				}
				if string([]byte{letters(x, y-1), letters(x, y-2), letters(x, y-3)}) == "MAS" {
					part1++
				}
				if string([]byte{letters(x-1, y), letters(x-2, y), letters(x-3, y)}) == "MAS" {
					part1++
				}
				if string([]byte{letters(x-1, y+1), letters(x-2, y+2), letters(x-3, y+3)}) == "MAS" {
					part1++
				}
				if string([]byte{letters(x-1, y-1), letters(x-2, y-2), letters(x-3, y-3)}) == "MAS" {
					part1++
				}
			}
		}
	}

	// Part 2
	part2 := 0
	for y := 0; y < sy; y++ {
		for x := 0; x < sx; x++ {
			if letters(x, y) == 'A' {
				w := string([]byte{
					letters(x-1, y-1),
					letters(x+1, y+1),
					letters(x+1, y-1),
					letters(x-1, y+1),
				})
				if w == "MSMS" || w == "MSSM" || w == "SMMS" || w == "SMSM" {
					part2++
				}
			}
		}
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
