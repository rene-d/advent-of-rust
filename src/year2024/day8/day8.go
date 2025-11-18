// [Day 8: Resonant Collinearity](https://adventofcode.com/2024/day/8)

package main

import (
	"fmt"
	"log"
	"os"
	"strings"
	"time"
)

// Point represents a 2D coordinate.
type Point struct {
	X, Y int
}

func solve(data []byte) (int, int) {

	lines := strings.Split(strings.TrimSpace(string(data)), "\n")
	width := len(lines[0])
	height := len(lines)

	// Collect all antennas by their character
	antennas := make(map[rune][]Point)
	for y, line := range lines {
		for x, c := range line {
			if c != '.' {
				antennas[c] = append(antennas[c], Point{X: x, Y: y})
			}
		}
	}

	// Part 1: Find antinodes (only the immediate neighbors)
	uniq := make(map[Point]bool)
	for _, positions := range antennas {
		// Generate all pairs of antennas
		for i := 0; i < len(positions); i++ {
			for j := i + 1; j < len(positions); j++ {
				a := positions[i]
				b := positions[j]

				// Antinode on one side of a
				antinode1 := Point{
					X: a.X - (b.X - a.X),
					Y: a.Y - (b.Y - a.Y),
				}
				uniq[antinode1] = true

				// Antinode on one side of b
				antinode2 := Point{
					X: b.X + (b.X - a.X),
					Y: b.Y + (b.Y - a.Y),
				}
				uniq[antinode2] = true
			}
		}
	}

	count := 0
	for p := range uniq {
		if p.X >= 0 && p.X < width && p.Y >= 0 && p.Y < height {
			count++
		}
	}

	// Part 2: Find all antinodes along the line (indefinitely)
	uniq = make(map[Point]bool)
	for _, positions := range antennas {
		// Generate all pairs of antennas
		for i := 0; i < len(positions); i++ {
			for j := i + 1; j < len(positions); j++ {
				a := positions[i]
				b := positions[j]

				dx := b.X - a.X
				dy := b.Y - a.Y

				// Extend in one direction (from a)
				for n := 0; ; n++ {
					x := a.X - n*dx
					y := a.Y - n*dy
					if !(x >= 0 && x < width && y >= 0 && y < height) {
						break
					}
					uniq[Point{X: x, Y: y}] = true
				}

				// Extend in the other direction (from b)
				for n := 0; ; n++ {
					x := b.X + n*dx
					y := b.Y + n*dy
					if !(x >= 0 && x < width && y >= 0 && y < height) {
						break
					}
					uniq[Point{X: x, Y: y}] = true
				}
			}
		}
	}

	return count, len(uniq)
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
