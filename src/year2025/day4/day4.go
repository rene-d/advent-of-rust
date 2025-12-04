// [Day 4: Printing Department](https://adventofcode.com/2025/day/4)

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

	grid := make([][]rune, len(lines))
	for i, row := range lines {
		grid[i] = []rune(row)
	}
	sx := len(grid[0])
	sy := len(grid)

	const PAPER_ROLL = '@'
	const EMPTY = '.'

	part1 := 0
	part2 := 0

	dirs := [8][2]int{
		{-1, -1}, {-1, 0}, {-1, 1},
		{0, -1}, {0, 1},
		{1, -1}, {1, 0}, {1, 1},
	}

	for {
		accessible := make(map[[2]int]struct{})

		for y := 0; y < sy; y++ {
			for x := 0; x < sx; x++ {
				if grid[y][x] == PAPER_ROLL {
					rolls := 0
					for _, d := range dirs {
						nx, ny := x+d[0], y+d[1]
						if nx >= 0 && nx < sx && ny >= 0 && ny < sy {
							if grid[ny][nx] == PAPER_ROLL {
								rolls++
							}
						}
					}
					if rolls < 4 {
						accessible[[2]int{x, y}] = struct{}{}
					}
				}
			}
		}

		if len(accessible) == 0 {
			break
		}

		if part1 == 0 {
			part1 = len(accessible)
		}

		part2 += len(accessible)

		for pos := range accessible {
			x, y := pos[0], pos[1]
			grid[y][x] = EMPTY
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
