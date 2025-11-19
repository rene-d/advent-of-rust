// [Day 14: Restroom Redoubt](https://adventofcode.com/2024/day/14)
package main

import (
	"fmt"
	"log"
	"os"
	"strings"
	"time"
)

type Robot struct {
	px, py int
	vx, vy int
}

type Puzzle struct {
	robots []Robot
	width  int
	height int
}

func newPuzzle(data []byte) Puzzle {
	lines := strings.Split(strings.TrimSpace(string(data)), "\n")

	var robots []Robot
	for _, line := range lines {
		if line == "" {
			continue
		}

		var robot Robot
		fmt.Sscanf(line, "p=%d,%d v=%d,%d", &robot.px, &robot.py, &robot.vx, &robot.vy)
		robots = append(robots, robot)
	}

	return Puzzle{
		robots: robots,
		width:  101,
		height: 103,
	}
}

func mod(a, b int) int {
	r := a % b
	if r < 0 {
		r += b
	}
	return r
}

func (p Puzzle) part1() int {
	quadrants := make(map[[2]int]int)

	for _, robot := range p.robots {
		px := mod(robot.px+robot.vx*100, p.width)
		py := mod(robot.py+robot.vy*100, p.height)

		if px == p.width/2 || py == p.height/2 {
			continue
		}

		q := [2]int{(px * 2) / p.width, (py * 2) / p.height}
		quadrants[q]++
	}

	product := 1
	for _, count := range quadrants {
		product *= count
	}
	return product
}

func (p Puzzle) part2() int {
outer:
	for seconds := 0; seconds < 100_000; seconds++ {
		grid := make(map[[2]int]struct{})

		for _, robot := range p.robots {
			px := mod(robot.px+robot.vx*seconds, p.width)
			py := mod(robot.py+robot.vy*seconds, p.height)

			key := [2]int{px, py}
			if _, exists := grid[key]; exists {
				continue outer
			}
			grid[key] = struct{}{}
		}

		horizontalLines := 0
		for y := 0; y < p.height; y++ {
			for x := 0; x < p.width-10; x++ {
				line := true
				for i := x; i < x+5; i++ {
					if _, ok := grid[[2]int{i, y}]; !ok {
						line = false
						break
					}
				}
				if line {
					horizontalLines++
				}
			}
		}

		if horizontalLines > 5 {
			return seconds
		}
	}

	return 0
}

func solve(data []byte) (int, int) {
	puzzle := newPuzzle(data)
	return puzzle.part1(), puzzle.part2()
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
